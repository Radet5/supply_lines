use bevy::{math::NormedVectorSpace, prelude::*};
use big_brain::{
    BigBrainPlugin, BigBrainSet,
    prelude::{ActionBuilder, ActionState, ScorerBuilder},
    scorers::Score,
    thinker::{ActionSpan, Actor, HasThinker, ScorerSpan},
};

use crate::{
    navigation::{EntityPath, FindPathEvent},
    time_control::{self, TimeController},
    vegetation::Fruit,
};

pub struct NeedsPlugin;

impl Plugin for NeedsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Satiety>()
            .insert_resource(NeedsTimer {
                timer: Timer::from_seconds(60., TimerMode::Repeating),
            })
            .add_systems(FixedUpdate, drain_needs)
            .add_plugins(BigBrainPlugin::new(PreUpdate))
            .add_systems(
                PreUpdate,
                (
                    (eat_action_system, move_to_nearest_system::<Fruit>)
                        .in_set(BigBrainSet::Actions),
                    satiety_scorer_system.in_set(BigBrainSet::Scorers),
                ),
            );
    }
}

#[derive(Resource, Debug)]
pub struct NeedsTimer {
    timer: Timer,
}

#[derive(Component, Debug, Reflect)]
pub struct Satiety {
    pub value: f32,
    pub drain_speed_per_sec: f32,
}

impl Default for Satiety {
    fn default() -> Self {
        Self {
            value: 100.0,
            drain_speed_per_sec: 100. / 8. / 60. / 60.,
        }
    }
}

#[derive(Component, Debug)]
#[require(Satiety)]
pub struct PhysicalNeeds;

impl Default for PhysicalNeeds {
    fn default() -> Self {
        Self
    }
}

fn drain_needs(mut query: Query<&mut Satiety>, time_controller: Res<TimeController>) {
    for mut satiety in query.iter_mut() {
        if satiety.value == 0. {
            return;
        } else if satiety.value > 0. {
            satiety.value -=
                (time_controller.scaled_delta().as_secs_f32()) * satiety.drain_speed_per_sec;
        } else {
            satiety.value = 0.;
        }
    }
}

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct Eat {
    until: f32,
    per_second: f32,
}

impl Eat {
    pub fn new(until: f32, per_second: f32) -> Self {
        Self { until, per_second }
    }
}

#[derive(Debug, Clone, Component, ActionBuilder)]
#[action_label = "MyGenericLabel"]
pub struct MoveToNearest<T: Component + std::fmt::Debug + Clone> {
    // We use a PhantomData to store the type of the component we're moving to.
    _marker: std::marker::PhantomData<T>,
}

impl<T: Component + std::fmt::Debug + Clone> MoveToNearest<T> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

pub fn move_to_nearest_system<T: Component + std::fmt::Debug + Clone>(
    mut query: Query<&mut Transform, With<T>>,
    thinkers: Query<(&Transform, Has<EntityPath>, Entity), (With<HasThinker>, Without<T>)>,
    mut action_query: Query<(&Actor, &mut ActionState, &ActionSpan), With<MoveToNearest<T>>>,
    mut find_path_event_writer: EventWriter<FindPathEvent>,
) {
    for (actor, mut action_state, span) in &mut action_query {
        let _guard = span.span().enter();

        match *action_state {
            ActionState::Requested => {
                debug!("Let's go find a {:?}", std::any::type_name::<T>());

                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                let (actor_transform, has_path, entity) = thinkers.get(actor.0).unwrap();
                // The goal is the nearest entity with the specified component.
                let goal_transform = query
                    .iter_mut()
                    .map(|t| (t.translation, t))
                    .min_by(|(a, _), (b, _)| {
                        let dist_a = actor_transform.translation.distance_squared(*a);
                        let dist_b = actor_transform.translation.distance_squared(*b);
                        dist_a.partial_cmp(&dist_b).unwrap()
                    })
                    .and_then(|t| Some(t.1));
                let Some(goal_transform) = goal_transform else {
                    continue;
                };
                let distance = actor_transform
                    .translation
                    .distance_squared(goal_transform.translation);

                trace!("Distance: {}", distance);

                if distance > 1.1 {
                    trace!("Stepping closer.");

                    // GET PATH
                    if !has_path {
                        find_path_event_writer.send(FindPathEvent::new(
                            actor_transform.translation,
                            goal_transform.translation,
                            entity,
                        ));
                    }
                } else {
                    debug!("We got there!");

                    *action_state = ActionState::Success;
                }
            }
            ActionState::Cancelled => {
                *action_state = ActionState::Failure;
            }
            _ => {}
        }
    }
}

fn eat_action_system(
    time_controller: Res<TimeController>,
    mut satieties: Query<&mut Satiety>,
    mut query: Query<(&Actor, &mut ActionState, &Eat, &ActionSpan)>,
) {
    for (Actor(actor), mut state, eat, span) in &mut query {
        let _guard = span.span().enter();

        if let Ok(mut satiety) = satieties.get_mut(*actor) {
            match *state {
                ActionState::Requested => {
                    debug!("Time to eat!");
                    *state = ActionState::Executing;
                }
                ActionState::Executing => {
                    trace!("EAting...");
                    satiety.value += eat.per_second * time_controller.scaled_delta().as_secs_f32();
                    satiety.value = satiety.value.min(90.);
                    if satiety.value >= eat.until {
                        debug!("Done eating");
                        *state = ActionState::Success;
                    }
                }
                ActionState::Cancelled => {
                    debug!("Eating cancelled.. we failed, guys...");
                    *state = ActionState::Failure;
                }
                _ => {}
            }
        }
    }
}

#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct Hungry;

pub fn satiety_scorer_system(
    satieties: Query<&Satiety>,
    mut query: Query<(&Actor, &mut Score, &ScorerSpan), With<Hungry>>,
) {
    for (Actor(actor), mut score, span) in &mut query {
        if let Ok(satiety) = satieties.get(*actor) {
            let hunger = (100.0 - satiety.value).max(0.0) / 100.0;
            // println!("hunger score {}", hunger);
            score.set(hunger);
            if satiety.value <= 20.0 {
                span.span()
                    .in_scope(|| debug!("Hunger above threshold! Score: {}", hunger));
            };
        }
    }
}
