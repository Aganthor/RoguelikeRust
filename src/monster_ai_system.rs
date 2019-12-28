extern crate specs;
use specs::prelude::*;
use super::{Viewshed, Monster, Name, Position, Map};
extern crate rltk;
use rltk::{Point, console};

pub struct MonsterAI {}

impl <'a> System<'a> for MonsterAI {
    type SystemData = ( WriteExpect<'a, Map>,
                        ReadExpect<'a, Point>,
                        ReadStorage<'a, Viewshed>,
                        ReadStorage<'a, Monster>,
                        ReadStorage<'a, Name>,
                        WriteStorage<'a, Position>);

    fn run(&mut self, data : Self::SystemData) {
        let (map, player_pos, viewshed, monster, name, position) = data;

        for (viewshed,_monster, name) in (&viewshed, &monster, &name).join() {
            if viewshed.visible_tiles.contains(&*player_pos) {
                console::log(&format!("{} shouts insults!", name.name));
            }
        }
    }
}