//Implementing requests.h and requests.c

//-----------------------DO LATER---------------
// Dirn requests_chooseDirection(Elevator e) __attribute__((pure));

// int requests_shouldStop(Elevator e) __attribute__((pure));

// Elevator requests_clearAtCurrentFloor(Elevator e) __attribute__((pure));


pub fn requests_above(e: Elevator)->int{
    for f in e.floor+1..N_FLOORS{
        for btn in 0..N_BUTTONS{
            if e.requests[f][btn] {
                return 1;
            }
        }
    }
    return 0;
}

pub fn requests_below(e:Elevator) -> int{
    for f in 0..e.floor{
        for btn in 0..N_BUTTONS{
            if e.requests[f][btn] {
                return 1;
            }
        }
    }
    return 0;
}

pub fn requests_chooseDirection(e:Elevator)->Dirn{
    match e.dirn {
        D_up => {
            if requests_above(e) {return D_Up;};
            if requests_below(e) {return D_Down;};
            return D_Stop;
        },
        D_Down => ,
        D_Stop => {
            if requests_below(e) {return D_Down;};
            if requests_above(e) {return D_Up;};
            return D_Stop;
        },
        _ => D_Stop;             
    };
}


pub fn requests_shouldStop(e: Elevator)->int{
    switch(e.dirn){
    case D_Down:
        return
            e.requests[e.floor][B_HallDown] ||
            e.requests[e.floor][B_Cab]      ||
            !requests_below(e);
    case D_Up:
        return
            e.requests[e.floor][B_HallUp]   ||
            e.requests[e.floor][B_Cab]      ||
            !requests_above(e);
    case D_Stop:
    default:
        return 1;
    }
}


Elevator requests_clearAtCurrentFloor(Elevator e){
        
    switch(e.config.clearRequestVariant){
    case CV_All:
        for(Button btn = 0; btn < N_BUTTONS; btn++){
            e.requests[e.floor][btn] = 0;
        }
        break;
        
    case CV_InDirn:
        e.requests[e.floor][B_Cab] = 0;
        switch(e.dirn){
        case D_Up:
            e.requests[e.floor][B_HallUp] = 0;
            if(!requests_above(e)){
                e.requests[e.floor][B_HallDown] = 0;
            }
            break;
            
        case D_Down:
            e.requests[e.floor][B_HallDown] = 0;
            if(!requests_below(e)){
                e.requests[e.floor][B_HallUp] = 0;
            }
            break;
            
        case D_Stop:
        default:
            e.requests[e.floor][B_HallUp] = 0;
            e.requests[e.floor][B_HallDown] = 0;
            break;
        }
        break;
        
    default:
        break;
    }
    
    return e;
}










