 pub const N_FLOORS: u8 = 4;
 pub const N_BUTTONS: u8 = 3;

pub enum Dirn{ 
    D_Down = -1,
    D_Stop = 0,
    D_Up = 1
}

pub enum Button{ 
    B_HallUp,
    B_HallDown,
    B_Cab
}


pub struct ElevInputDevice{
    floorSensor: fn() -> int,
    requestButton: fn(int, Button) -> int,
    stopButton: fn() -> int,
    obstruction: fn() ->int
}

pub struct ElevOutputDevice {
    floorIndicator: fn(int) -> void,
    requestButtonLight: fn(int, Button, int) -> void,
    doorLight: fn(int) -> void,
    stopButtonLight: fn(int) -> void,
    motorDirection: fn(Dirn) -> void
}
