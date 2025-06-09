use crate::prelude::*;
use bevy::prelude::*;

const FRAME_DURATION: f32 = 0.15;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (animate_sprite_frames, animate_image_node_frames).in_set(PausableSystems),
    );
}

#[derive(Component, Default, Clone, PartialEq)]
pub struct AnimationFrameQueue {
    frames: &'static [usize],
    frame_override: Option<&'static [usize]>,
    current_index: usize,
    timer: Timer,
}

impl AnimationFrameQueue {
    pub fn new(frames: &'static [usize]) -> Self {
        assert!(!frames.is_empty(), "Animation frames cannot be empty");
        Self {
            frames,
            frame_override: None,
            current_index: 0,
            timer: Timer::from_seconds(FRAME_DURATION, TimerMode::Repeating),
        }
    }

    pub fn set_frames(&mut self, frames: &'static [usize]) {
        assert!(!frames.is_empty(), "Animation frames cannot be empty");
        self.frames = frames;
        self.current_index = 0;
        self.timer.reset();
    }

    pub fn set_override(&mut self, override_frames: &'static [usize]) {
        assert!(
            !override_frames.is_empty(),
            "Override frames cannot be empty"
        );
        self.frame_override = Some(override_frames);
        self.current_index = 0;
        self.timer.reset();
    }

    pub fn tick_and_advance(&mut self, time: &Time, sprite: &mut TextureAtlas) {
        self.timer.tick(time.delta());

        let active_frames = self.frame_override.unwrap_or(self.frames);

        if self.timer.just_finished() {
            // Set sprite BEFORE advancing index
            sprite.index = active_frames[self.current_index];
            self.current_index += 1;

            if self.current_index >= active_frames.len() {
                if self.frame_override.is_some() {
                    self.frame_override = None;
                    self.current_index = 0;
                } else {
                    self.current_index = 0;
                }
            }
        }
    }
}

fn animate_sprite_frames(
    time: Res<Time>,
    mut query: Query<(&mut AnimationFrameQueue, &mut Sprite)>,
) {
    for (mut queue, mut sprite) in &mut query {
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            queue.tick_and_advance(&time, atlas);
        } else {
            continue;
        }
    }
}

fn animate_image_node_frames(
    time: Res<Time>,
    mut query: Query<(&mut AnimationFrameQueue, &mut ImageNode)>,
) {
    for (mut queue, mut node) in &mut query {
        if let Some(atlas) = node.texture_atlas.as_mut() {
            queue.tick_and_advance(&time, atlas);
        } else {
            continue;
        }
    }
}
