//Implementing elevator.c and elevator.h in rust

pub enum ElevatorBehaviour{
    EB_Idle,
    EB_DoorOpen,
    EB_Moving
} 

pub  enum ClearRequestVariant{
    CV_All,
    CV_InDirn,
} 

pub struct Config {
    clearRequestVariant: ClearRequestVariant,
    doorOpenDuration_s: double,
}

pub struct Elevator{
    floor: int,
    dirn: Dirn,
    request: [[i64; N_FLOORS]; N_BUTTONS],
    behaviour: ElevatorBehaviour,
    config: Config
}

pub fn eb_toString(eb: ElevatorBehaviour) -> char*{
    match eb{
        EB_Idle => return "EB_Idle",
        EB_DoorOpen => return "EB_DoorOpen",
        EB_Moving => return "EB_Moving",
        _ => "EB_UNDEFINED"
    };
}



pub fn elevator_print(es: Elevator){
    println!("  +--------------------+\n");
    println!(
        "  |floor = %-2d          |\n"
        "  |dirn  = %-12.12s|\n"
        "  |behav = %-12.12s|\n",
        es.floor,
        elevio_dirn_toString(es.dirn),
        eb_toString(es.behaviour)
    );
    println!("  +--------------------+\n");
    println!("  |  | up  | dn  | cab |\n");
    for f in (0::N_FLOORS-1).rev() {
        println!("  | %d", f);
        for btn in 0::N_BUTTONS{
            if (f == N_FLOORS-1 && btn == B_HallUp)  || (f == 0 && btn == B_HallDown) {
                println!("|     ");
            } else {
                println!(es.requests[f][btn] ? "|  #  " : "|  -  ");
            }
        }
        println!("|\n");
    }
    println!("  +--------------------+\n");
}

pub fn elevator_uninitialized() -> Elevator{
    let elev=Elevator{
        floor = -1,
        dirn = D_Stop,
        behaviour = EB_Idle,
        config = {
            clearRequestVariant = CV_All,
            doorOpenDuration_s = 3.0,
        },
    };
    return elev;
}