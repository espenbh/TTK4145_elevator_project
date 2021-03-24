//Implementing elevator_io_devices.c in rust

//-----------------------DO LATER--------------------
// static void __attribute__((constructor)) elev_init(void){
//     elevator_hardware_init();
// }

// static int _wrap_requestButton(int f, Button b){
//     return elevator_hardware_get_button_signal(b, f);
// }
// static void _wrap_requestButtonLight(int f, Button b, int v){
//     elevator_hardware_set_button_lamp(b, f, v);
// }
// static void _wrap_motorDirection(Dirn d){
//     elevator_hardware_set_motor_direction(d);
// }


pub fn elevio_getInputDevice() -> ElevInputDevice {
    let myElevInputDevice = ElevInputDevice {
        floorSensor    = elevator_hardware_get_floor_sensor_signal,
        requestButton  = _wrap_requestButton,
        stopButton     = elevator_hardware_get_stop_signal,
        obstruction    = elevator_hardware_get_obstruction_signal
    };
    return myElevInputDevice
}


pub fn elevio_getOutputDevice() -> ElevOutputDevice{
    let myElevOutpuDevice= ElevOutputDevice {
        floorIndicator     = elevator_hardware_set_floor_indicator,
        requestButtonLight = _wrap_requestButtonLight,
        doorLight          = elevator_hardware_set_door_open_lamp,
        stopButtonLight    = elevator_hardware_set_stop_lamp,
        motorDirection     = _wrap_motorDirection
    };
    return myElevOutpuDevice
}


pub fn elevio_dirn_toString(d: Dirn) -> char*{
    let elevio_dirn_string=match d{
        D_up => "D_Up",
        D_Down => "D_Down",
        D_Stop => "D_Stop",
        _ => "D_UNDEFINED"
    };
    return elevio_dirn_string;
}


elevio_button_toString(b: Button)->char* {
    let elevio_button_string=match b{
        B_HallUp => "B_HallUp",
        B_HallDown => "B_HallDown",
        B_Cab => "B_Cab",
        _ => "B_UNDEFINED"
    };
    return elevio_button_string;
}
