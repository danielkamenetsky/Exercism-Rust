#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    // Check to ensure that player is indeed dead (health 0), if so return a new player instance with health 100
    // If level of player is 10 or more they should be revived with 100 mana, otherwise mana is none but they preserve their level
    // If players health is indeed 1 or more, then method should return none
    pub fn revive(&self) -> Option<Player> {
        let mut player = Player { health: self.health, mana: self.mana, level: self.level};
        if player.health == 0 {
            player.health = 100;
            if player.level >= 10 {
                player.mana = Some(100);
            }
            else { 
                player.mana = None
            }
            Some(player)
        }
        else {
            return None
        }

    }
    //Takes in a mutable reference to the player, mana_cost parameter, returns amount of damage (2x mana cost)
    
    // 1. If the player does not have access to a mana pool, attempting 
    //to cast the spell must decrease their health by the mana cost of the spell. The damage returned must be 0.
    
    //2. If the player has a mana pool but insufficient mana, the method should not affect the pool,
    // but instead return 0

    //3. Otherwise, the `mana_cost` should be deducted from the Player's
    // mana pool and the appropriate amount of damage should be returned.

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if self.mana == None { 
            self.health = self.health - 
            if self.health < mana_cost { self.health }
            else { mana_cost};
            return 0
        }
        else if self.mana < Some(mana_cost) {
            return 0
        }
        else {
            self.mana = Some(self.mana.unwrap() - mana_cost);
            return mana_cost * 2
         }
    }
}
