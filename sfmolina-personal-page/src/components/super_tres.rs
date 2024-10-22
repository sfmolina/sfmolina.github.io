//-------------------------------------------------------------------//
//  AUTHOR:    @sfmolina                                            //
//  Version:   v1                                                  //
//  Modified:  11ago24                                            //
//---------------------------------------------------------------//
// DESCRIPCIÓN:                                                
// Componente de un juego de super tres en raya.



//-------------------------------------------------------------------
// IMPORTS



use yew::prelude::*;
use std::collections::HashSet;



//-------------------------------------------------------------------
//-------------------------------------------------------------------
// DATA STRUCTURES


/// Enumeración que representa a los jugadores
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Player {
    First,
    Second,
}

/// Función que cambia de jugador
fn switch(op: Option<Player>) -> Option<Player> {
    match op {
        Some(Player::First) => Some(Player::Second),
        Some(Player::Second) => Some(Player::First),
        None => None,
    }
}

/// Estructura que representa una posición en el tablero
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl Position {

    /// Proporciona la posición total en el supertablero a partir de la posición en un subtablero
    fn total_pos(pos1: Position, pos2: Position) -> Position {
        Position { x: pos1.x * 3 + pos2.x, y: pos1.y * 3 + pos2.y }
    }

    fn partial_pos(total_pos: Position) -> (Position, Position) {
        (Position { x: total_pos.x / 3, y: total_pos.y / 3 }, Position { x: total_pos.x % 3, y: total_pos.y % 3 })
    }
}

/// Posición inalcanzable.
/// 
/// X = 3, Y = 3
/// 
const UNREACHABLE: Position = Position { x: 9, y: 9 };

/// Estructura que representa un tablero de 3x3
#[derive(Clone, PartialEq, Debug)]
struct TableroTres {
    data: [[Option<Player>; 3]; 3],
}

impl TableroTres {

    fn new() -> Self {
        Self {
            data: [[None; 3]; 3],
        }
    }

    fn put(&mut self, pos: Position, value: Player) {
        self.data[pos.x as usize][pos.y as usize] = Some(value);
    }

    fn reset(&mut self, pos: Position) {
        self.data[pos.x as usize][pos.y as usize] = None;
    }

    fn get(&self, pos: Position) -> Option<Player> {
        self.data[pos.x as usize][pos.y as usize]
    }

    fn check(&self) -> Option<Player> {

        let check_line = |line: [Option<Player>; 3]| -> Option<Player> {
            if line.iter().all(|x| x.is_some()) && line[0] == line[1] && line[1] == line[2] {
                // Si todos los elementos son iguales y no son None, devuelvo el valor, el tablero se completó
                line[0]
            } else {
                None
            }
        };
    
        // Check rows
        for i in 0..3 {
            if let Some(r) = check_line(self.data[i]) {
                return Some(r);
            }
        }
    
        // Check columns
        for i in 0..3 {
            if let Some(r) = check_line([self.data[0][i], self.data[1][i], self.data[2][i]]) {
                return Some(r);
            }
        }
    
        // Check diagonals
        if let Some(r) = check_line([self.data[0][0], self.data[1][1], self.data[2][2]]) {
            return Some(r);
        }
        if let Some(r) = check_line([self.data[0][2], self.data[1][1], self.data[2][0]]) {
            return Some(r);
        }
    
        None
    }

    fn playable(&self) -> bool {
        self.data.iter().any(|x| x.iter().any(|y| y.is_none()))
    }


}


/// Estructura que representa un supertablero de 3x3
pub struct TableroSuperTres {
    tablero: [[Result<Player, TableroTres>; 3]; 3],
}

impl TableroSuperTres {

    fn new() -> Self {

        let tablero = [
            [Err(TableroTres::new()), Err(TableroTres::new()), Err(TableroTres::new())],
            [Err(TableroTres::new()), Err(TableroTres::new()), Err(TableroTres::new())],
            [Err(TableroTres::new()), Err(TableroTres::new()), Err(TableroTres::new())],
        ];

        Self {
            tablero,
        }
    }

    fn get(&self, pos1: Position) -> Result<&Player, &TableroTres> {
        self.tablero[pos1.x as usize][pos1.y as usize].as_ref()
    }

    fn get_mut(&mut self, pos1: Position) -> Result<&mut Player, &mut TableroTres> {
        self.tablero[pos1.x as usize][pos1.y as usize].as_mut()
    }

    fn put(&mut self, pos1: Position, pos2: Position, value: Player) {
        let tab = self.get_mut(pos1);

        match tab {
            Err(tab3x3) => {
                // Si tengo un tablero, lo modifico
                tab3x3.put(pos2, value);
            },
            Ok(_) => {
                // Si tengo un valor, el tablero ya se completó, no hago nada
            },
        }
        
    }

    fn reset(&mut self, pos1: Position, pos2: Position) {
        let tab = self.get_mut(pos1);

        match tab {
            Err(tab3x3) => {
                // Si tengo un tablero, lo modifico
                tab3x3.reset(pos2);
            },
            Ok(_) => {
                // Si tengo un valor, el tablero ya se completó, no hago nada
            },
        }
    }

    /// Chequea si el supertablero está completo.
    /// Si el supertablero está completo, devuelve el valor.
    /// 
    /// Los subtableros se chequean y si están completos, se cambian por el valor
    fn check(&mut self) -> Option<Player> {

        // Primero chequeo cada subtablero por si está completo y sustituyo dicho subtablero del supertablero
        // por el valor ganador
        for i in 0..3 {
            for j in 0..3 {
                if let Err(tab) = &self.tablero[i][j] {
                    if let Some(value) = tab.check() {
                        self.tablero[i][j] = Ok(value);
                    }
                }
            }
        }

        // Después, chequeo si el supertablero está completo y devuelvo el valor ganador

        let check_line = |line: &[Result<Player, TableroTres>; 3]| -> Option<Player> {
            if line.iter().all(|x| x.is_ok()) {
                let first_value = line[0].as_ref().ok()?;
                if line.iter().all(|x| x.as_ref().ok() == Some(first_value)) {
                    // Si todos los elementos son iguales y no son None, devuelvo el valor, el tablero se completó
                    return Some(*first_value);
                }
            }
            None
        };

        // Check rows
        for i in 0..3 {
            if let Some(r) = check_line(&self.tablero[i]) {
                return Some(r);
            }
        }

        // Check columns
        for i in 0..3 {
            if let Some(r) = check_line(&[self.tablero[0][i].clone(), self.tablero[1][i].clone(), self.tablero[2][i].clone()]) {
                return Some(r);
            }
        }

        // Check diagonals
        if let Some(r) = check_line(&[self.tablero[0][0].clone(), self.tablero[1][1].clone(), self.tablero[2][2].clone()]) {
            return Some(r);
        }
        if let Some(r) = check_line(&[self.tablero[0][2].clone(), self.tablero[1][1].clone(), self.tablero[2][0].clone()]) {
            return Some(r);
        }

        None

    }

}


//-------------------------------------------------------------------
//-------------------------------------------------------------------
// COMPONENT


/// Mensajes que puede recibir el componente
pub enum SuperTresMsg {
    Mark(Position, Position),
    Check,
}


/// Información que guarda el componente
pub struct SuperTresComponent {

    /// Supertablero de juego
    tablero: TableroSuperTres,

    /// Jugador al que le toca jugar.
    /// Si es None, el juego terminó.
    turn: Option<Player>,

    /// Indica si se jugó en el último turno.
    turn_played: bool,

    /// Última casilla jugada
    last_played: Option<Position>,

    /// Conjunto de posiciones jugadas.
    /// Las posiciones jugadas no se pueden volver a jugar.
    /// 
    /// Son posiciones totales en el supertablero, NO se refieren a subtableros.
    played_total_positions: HashSet<Position>,

    /// Última posición total jugada.
    last_total_played: Option<Position>,

    /// Subtablero activo.
    /// Si es None, cualquier subtablero está activo.
    active_table: Option<Position>,

    /// Ganador
    winner: Option<Player>,
}


/// Implementación del componente
impl Component for SuperTresComponent {


    type Message = SuperTresMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {

        
        Self { 
            tablero: TableroSuperTres::new(),
            turn: Some(Player::First),
            turn_played: false,
            last_played: None,
            played_total_positions: HashSet::new(),
            last_total_played: None,
            active_table: Some(Position { x: 1, y: 1 }),
            winner: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {

        match msg {
            SuperTresMsg::Mark(position1, position2) => {

                // Compruebo si el juego continúa
                if let Err(tab) = self.tablero.get(position1) {

                    // El juego continúa, marco la casilla

                    match self.turn {

                        // Turno del jugador 1
                        Some(Player::First) => {
                            
                            // Compruebo la casilla que se quiere marcar
                            match tab.get(position2) {
                                None => {

                                    marcar_casilla_libre(self, position1, position2, Player::First)

                                },
                                Some(Player::First) => {

                                    marcar_casilla_ocupada(self, position1, position2)
                                    
                                },
                                Some(Player::Second) => {
                                    // Si ya está marcado por el otro jugador, no se hace nada
                                    false
                                }
                            }

                        },
                        // Turno del jugador 2
                        Some(Player::Second) => {
                            
                            // Compruebo la casilla que se quiere marcar
                            match tab.get(position2) {
                                None => {

                                    marcar_casilla_libre(self, position1, position2, Player::Second)

                                },
                                Some(Player::Second) => {

                                    marcar_casilla_ocupada(self, position1, position2)

                                },
                                Some(Player::First) => {
                                    // Si ya está marcado por el otro jugador, no se hace nada
                                    false
                                }
                            }
                        },
                        // Ningún jugador
                        None => {
                            // El juego ya terminó, no se hace nada
                            false
                        },
                    }

                } else {

                    // El juego ya terminó, no se hace nada

                    false
                }

            },
            SuperTresMsg::Check => {

                let final_result = self.tablero.check();

                match final_result {
                    None => {
                        
                        // El juego continúa

                        // Si no se jugó, no se hace nada
                        if self.turn_played {
                
                            // El subtablero en el que quiero jugar es el que corresponde a la última casilla jugada
                            let future_table = self.tablero.get(self.last_played.unwrap());
                            
                            if let Err(future_table) = future_table {
                                // Si el tablero no está completo, el tablero activo es el subtablero correspondiente
                                if future_table.playable() {
                                    self.active_table = self.last_played;
                                } else {
                                    // Si el tablero no tiene casillas libres y no es completo,
                                    // es un empate y el tablero activo es el super tablero completo
                                    self.active_table = None;
                                }
                            } else {
                                // Si el tablero está completo, el tablero activo es el supertablero completo
                                self.active_table = None;
                            }

                            // Cambio de turno
                            self.turn = switch(self.turn);
                            self.turn_played = false;

                            // Se almacena la posición total jugada y se limpia la última posición total jugada
                            self.played_total_positions.insert(self.last_total_played.unwrap());
                            self.last_total_played = None;

                        }

                    },
                    Some(value) => {

                        // El juego terminó

                        // Si el supertablero está completo, el tablero activo es inalcanzable.
                        // El turno es None.
                        // El ganador es el valor del supertablero.

                        self.active_table = Some(UNREACHABLE);
                        self.turn = None;
                        self.winner = Some(value);
                    },
                }

                true
            },
            
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <table class="super-tres">
                <table>
                    <thead>
                    <button class="check-button" onclick={ctx.link().callback(move |_| SuperTresMsg::Check)}>{"Check"}</button>
                    </thead>
                    <tbody>
                        <tr>
                            <td class={
                                match self.winner {
                                    Some(Player::First) => "first-player",
                                    Some(Player::Second) => "second-player",
                                    None => "",
                                }
                            }>
                                <table class="super-3x3">
                                    { for (0..3).map(|i| html! {
                                        <tr>
                                            { for (0..3).map(|j| html! {
                                                match self.tablero.get(Position { x: i, y: j }) {
                                                    // Si el tablero está completo, muestro el valor
                                                    Ok(Player::First) => {
                                                        html! {
                                                            
                                                            <td class="first-player">{"X"}</td>
                                                            
                                                        }
                                                    },
                                                    Ok(Player::Second) => {
                                                        html! {
                                                            
                                                            <td class="second-player">{"O"}</td>
                                                            
                                                        }
                                                    },
                                                    Err(tab) => {
                                                        html! {
                                                            // Si el tablero no está completo, muestro el tablero
                                                            <td class={
                                                                let actual_table = Position { x: i, y: j };
                                                                if !disabled_table(self.active_table, actual_table) {
                                                                    "playable"
                                                                } else {
                                                                    ""
                                                                }
                                                            }>
                                                                <table class="tabla-3x3">
                                                                    { for (0..3).map(|k| html! {
                                                                        <tr>
                                                                            { 
                                                                                for (0..3).map(|l| html! {

                                                                                    match tab.get(Position { x: k, y: l }) {

                                                                                        Some(Player::First) => {
                                                                                            html! {
                                                                                                <td class="first-player">
                                                                                                    <button disabled={
                                                                                                        let actual_table = Position { x: i, y: j };
                                                                                                        disabled_table(self.active_table, actual_table)
                                                                                                    } onclick={ctx.link().callback(move |_| SuperTresMsg::Mark(Position { x: i, y: j }, Position { x: k, y: l }))}>{"X"}</button>
                                                                                                </td>
                                                                                            }
                                                                                        },
                                                                                        Some(Player::Second) => {
                                                                                            html! {
                                                                                                <td class="second-player">
                                                                                                    <button disabled={
                                                                                                        let actual_table = Position { x: i, y: j };
                                                                                                        disabled_table(self.active_table, actual_table)
                                                                                                    } onclick={ctx.link().callback(move |_| SuperTresMsg::Mark(Position { x: i, y: j }, Position { x: k, y: l }))}>{"O"}</button>
                                                                                                </td>
                                                                                            }
                                                                                        },
                                                                                        None => {
                                                                                            html! {
                                                                                                <td class="unplayed">
                                                                                                    <button disabled={
                                                                                                        let actual_table = Position { x: i, y: j };
                                                                                                        disabled_table(self.active_table, actual_table)
                                                                                                    } onclick={ctx.link().callback(move |_| SuperTresMsg::Mark(Position { x: i, y: j }, Position { x: k, y: l }))}>{" "}</button>
                                                                                                </td>
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    
                                                                                })
                                                                            }
                                                                        </tr>
                                                                    }) }
                                                                </table>
                                                            </td>
                                                        }
                                                    }
                                                }
                                            }) }
                                        </tr>
                                    }) }
                                </table>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </table>
        }
    }

}


fn disabled_table(active_table: Option<Position>, actual_table: Position) -> bool {
    (active_table.is_some())&&(active_table.unwrap() != actual_table)
}

fn marcar_casilla_libre(super_tres: &mut SuperTresComponent, pos1: Position, pos2: Position, player: Player) -> bool {
    if super_tres.turn_played {
        // Si ya se jugó y no se hizo check, se revierte la jugada anterior
        // para que no puedas marcar varias casillas en un solo turno
        let (last_table, _) = Position::partial_pos(super_tres.last_total_played.unwrap());
        super_tres.tablero.reset(last_table, super_tres.last_played.unwrap());
        //self.last_total_played = None;
    }

    // Si no está marcado, se marca
    // El siguiente tablero activo es el subtablero correspondiente a la casilla marcada
    // Se jugó el turno

    super_tres.tablero.put(pos1, pos2, player);
    super_tres.last_played = Some(pos2);
    // Se guarda la posición total jugada
    super_tres.last_total_played = Some(Position::total_pos(pos1, pos2));
    super_tres.turn_played = true;

    true
}

fn marcar_casilla_ocupada(super_tres: &mut SuperTresComponent, pos1: Position, pos2: Position) -> bool {
    // Compruebo si la casilla ya fue jugada por el mismo jugador anteriormente
    if !super_tres.played_total_positions.contains(&Position::total_pos(pos1, pos2)) {
                                        
        // Si ya está marcado, se resetea
        // Si se resetea, se vuelve a jugar en el mismo tablero
        // No se jugó el turno

        super_tres.tablero.reset(pos1, pos2);
        super_tres.last_played = Some(pos1);
        super_tres.turn_played = false;
        // No hay posición total jugada
        super_tres.last_total_played = None;

        true
    } else {
        // Si ya está marcado anteriormente por el mismo jugador, no se hace nada
        false
    }
}
