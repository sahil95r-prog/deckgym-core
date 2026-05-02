#[cfg(test)]
mod tests {
    use deckgym_core::{
        actions::{Action, SimpleAction},
        models::EnergyType,
        state::State,
        utils::{get_card_by_enum, to_playable_card},
        CardId,
        Game,
    };

    /// Training attaches 1 Colorless Energy from the Energy Zone to Kubfu.
    #[test]
    fn test_kubfu_training_attaches_colorless_energy() {
        let kubfu = get_card_by_enum(CardId::B3097Kubfu);
        let opponent = get_card_by_enum(CardId::A1001Bulbasaur);

        let mut state = State::default();
        state.in_play_pokemon[0][0] = Some(to_playable_card(&kubfu, false));
        state.in_play_pokemon[1][0] = Some(to_playable_card(&opponent, false));

        let mut game = Game::new(state);
        let actions = game.generate_possible_actions();
        let attack_action = actions
            .iter()
            .find(|a| matches!(a.action, SimpleAction::Attack(0)))
            .expect("Attack(0) should be available");

        game.apply_action(attack_action.clone());

        let kubfu_in_play = game.get_state_clone().get_active(0);
        assert_eq!(
            kubfu_in_play.attached_energy,
            vec![EnergyType::Colorless],
            "Training should attach 1 Colorless Energy to Kubfu"
        );
    }

    /// Training does 0 damage to the opponent.
    #[test]
    fn test_kubfu_training_does_no_damage() {
        let kubfu = get_card_by_enum(CardId::B3097Kubfu);
        let opponent = get_card_by_enum(CardId::A1001Bulbasaur); // 60 HP

        let mut state = State::default();
        state.in_play_pokemon[0][0] = Some(to_playable_card(&kubfu, false));
        state.in_play_pokemon[1][0] = Some(to_playable_card(&opponent, false));

        let mut game = Game::new(state);
        let actions = game.generate_possible_actions();
        let attack_action = actions
            .iter()
            .find(|a| matches!(a.action, SimpleAction::Attack(0)))
            .expect("Attack(0) should be available");

        game.apply_action(attack_action.clone());

        assert_eq!(
            game.get_state_clone().get_active(1).get_remaining_hp(),
            60,
            "Training should deal 0 damage"
        );
    }
}
