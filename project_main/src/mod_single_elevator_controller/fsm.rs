use elevator_driver::elevio;
use elevator_driver::elevio::elev as e;
mod 

pub fn setAllLights(es: e::Elevator){
    for floor in 0..e::N_FLOORS {
        for btn in 0..e::N_BUTTONS {
            es.call_button_light(floor, btn, true);
        }
    }
}

pub fn fsm_onInitBetweenFloors(){
    outputDevice.motorDirection(D_Down);
    elevator.dirn=D_Down;
    elevator.behaviour=EB_Moving;
}


pub fn fsm_onInitBetweenFloors(){
    outputDevice.motorDirection(D_Down);
    elevator.dirn = D_Down;
    elevator.behaviour = EB_Moving;
}

pub fn fsm_onRequestButtonPress(btn_floor: int, btn_type: Button){
    println!("\n\nfsm_onRequestButtonPress(%d, %s)\n", btn_floor, elevio_button_toString(btn_type));
    elevator_print(elevator);

    match elevator.behaviour {
        EB_DoorOpen => {
            if(elevator.floor == btn_floor){
                timer_start(elevator.config.doorOpenDuration_s);
            } else {
                elevator.requests[btn_floor][btn_type] = 1;
            }
        },
        EB_Moving => {
            elevator.requests[btn_floor][btn_type] = 1;
        },

        EB_Idle => {
            if(elevator.floor == btn_floor){
                outputDevice.doorLight(1);
                timer_start(elevator.config.doorOpenDuration_s);
                elevator.behaviour = EB_DoorOpen;
            } else {
                elevator.requests[btn_floor][btn_type] = 1;
                elevator.dirn = requests_chooseDirection(elevator);
                outputDevice.motorDirection(elevator.dirn);
                elevator.behaviour = EB_Moving;
            }
        },
    }

    setAllLights(elevator);

    println!("\nNew state:\n");
    elevator_print(elevator);
}


pub fn fsm_onFloorArrival(newFloor: int){
    println!("\n\nfsm_onFloorArrival(%d)\n", newFloor);
    elevator_print(elevator);

    elevator.floor = newFloor;

    outputDevice.floorIndicator(elevator.floor);

    match elevator.behaviour {
        EB_Moving => {
            if(requests_shouldStop(elevator)){
                outputDevice.motorDirection(D_Stop);
                outputDevice.doorLight(1);
                elevator = requests_clearAtCurrentFloor(elevator);
                timer_start(elevator.config.doorOpenDuration_s);
                setAllLights(elevator);
                elevator.behaviour = EB_DoorOpen;
            };
        },
    };
    println!("\nNew state:\n");
    elevator_print(elevator);
}


    

pub fn fsm_onDoorTimeout(){
    println!("\n\nfsm_onDoorTimeout()\n");
    elevator_print(elevator);

    match elevator.behaviour{
    EB_DoorOpen => {
        elevator.dirn = requests_chooseDirection(elevator);

        outputDevice.doorLight(0);
        outputDevice.motorDirection(elevator.dirn);

        if(elevator.dirn == D_Stop){
            elevator.behaviour = EB_Idle;
        } else {
            elevator.behaviour = EB_Moving;
        }
    }
    default:
        break;
    }

    printf("\nNew state:\n");
    elevator_print(elevator);
}
