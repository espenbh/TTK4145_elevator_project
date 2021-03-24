//Implementing timer.c in rust
use std::time::Duration;
use std::time::SystemTime;

pub fn get_wall_time() -> double{
    return SystemTime::now();
}

timerEndTime: double;
timerActive: int;

pub fn timer_start(duration: double){
    timerEndTime    = get_wall_time() + duration;
    timerActive     = 1;
}

pub fn timer_stop(){
    timerActive = 0;
}

pub fn timer_timedOut()->int{
    return (timerActive  &&  get_wall_time() > timerEndTime);
}


