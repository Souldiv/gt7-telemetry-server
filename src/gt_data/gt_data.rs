use byteorder::{ByteOrder, LittleEndian};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct GTData {
    package_id: i32,
    best_lap: i32,
    last_lap: i32,
    current_lap: i16,
    current_gear: u16,
    suggested_gear: u16,
    fuel_capacity: f32,
    current_fuel: f32,
    boost: f32,
    tyre_diameter_FL: f32,
    tyre_diameter_FR: f32,
    tyre_diameter_RL: f32,
    tyre_diameter_RR: f32,
    type_speed_FL: f32,
    type_speed_FR: f32,
    type_speed_RL: f32,
    tyre_speed_RR: f32,
    car_speed: f32,
    tyre_slip_ratio_FL: String,
    tyre_slip_ratio_FR: String,
    tyre_slip_ratio_RL: String,
    tyre_slip_ratio_RR: String,
    time_on_track: Duration,
    total_laps: i16,
    current_position: i16,
    total_positions: i16,
    car_id: i32,
    throttle: f32,
    rpm: f32,
    rpm_rev_warning: u16,
    brake: f32,
    rpm_rev_limiter: u16,
    estimated_top_speed: i16,
    clutch: f32,
    clutch_engaged: f32,
    rpm_after_clutch: f32,
    oil_temp: f32,
    water_temp: f32,
    oil_pressure: f32,
    ride_height: f32,
    tyre_temp_FL: f32,
    tyre_temp_FR: f32,
    suspension_fl: f32,
    suspension_fr: f32,
    tyre_temp_rl: f32,
    tyre_temp_rr: f32,
    suspension_rl: f32,
    suspension_rr: f32,
    gear_1: f32,
    gear_2: f32,
    gear_3: f32,
    gear_4: f32,
    gear_5: f32,
    gear_6: f32,
    gear_7: f32,
    gear_8: f32,
    position_x: f32,
    position_y: f32,
    position_z: f32,
    velocity_x: f32,
    velocity_y: f32,
    velocity_z: f32,
    rotation_pitch: f32,
    rotation_yaw: f32,
    rotation_roll: f32,
    angular_velocity_x: f32,
    angular_velocity_y: f32,
    angular_velocity_z: f32,
    is_paused: bool,
    in_race: bool,
}

impl GTData {
    pub fn new(ddata: &[u8]) -> Self {
        let mut data = GTData::default();

        data.package_id = LittleEndian::read_i32(&ddata[0x70..0x74]);
        data.best_lap = LittleEndian::read_i32(&ddata[0x78..0x7C]);
        data.last_lap = LittleEndian::read_i32(&ddata[0x7C..0x80]);
        data.current_lap = LittleEndian::read_i16(&ddata[0x74..0x76]);
        data.current_gear = (LittleEndian::read_u16(&ddata[0x90..0x92]) & 0b00001111);
        data.suggested_gear = (LittleEndian::read_u16(&ddata[0x90..0x92]) >> 4);
        data.fuel_capacity = LittleEndian::read_f32(&ddata[0x48..0x4C]);
        data.current_fuel = LittleEndian::read_f32(&ddata[0x44..0x48]);
        data.boost = LittleEndian::read_f32(&ddata[0x50..0x54]) - 1.0;

        data.tyre_diameter_FL = LittleEndian::read_f32(&ddata[0xB4..0xB8]);
        data.tyre_diameter_FR = LittleEndian::read_f32(&ddata[0xB8..0xBC]);
        data.tyre_diameter_RL = LittleEndian::read_f32(&ddata[0xBC..0xC0]);
        data.tyre_diameter_RR = LittleEndian::read_f32(&ddata[0xC0..0xC4]);

        data.type_speed_FL =
            (3.6 * data.tyre_diameter_FL * LittleEndian::read_f32(&ddata[0xA4..0xA8])).abs();
        data.type_speed_FR =
            (3.6 * data.tyre_diameter_FR * LittleEndian::read_f32(&ddata[0xA8..0xAC])).abs();
        data.type_speed_RL =
            (3.6 * data.tyre_diameter_RL * LittleEndian::read_f32(&ddata[0xAC..0xB0])).abs();
        data.tyre_speed_RR =
            (3.6 * data.tyre_diameter_RR * LittleEndian::read_f32(&ddata[0xB0..0xB4])).abs();

        data.car_speed = 3.6 * LittleEndian::read_f32(&ddata[0x4C..0x50]);

        if data.car_speed > 0.0 {
            data.tyre_slip_ratio_FL = format!("{:6.2}", data.type_speed_FL / data.car_speed);
            data.tyre_slip_ratio_FR = format!("{:6.2}", data.type_speed_FR / data.car_speed);
            data.tyre_slip_ratio_RL = format!("{:6.2}", data.type_speed_RL / data.car_speed);
            data.tyre_slip_ratio_RR = format!("{:6.2}", data.tyre_speed_RR / data.car_speed);
        } else {
            data.tyre_slip_ratio_FL = "0.00".to_string();
            data.tyre_slip_ratio_FR = "0.00".to_string();
            data.tyre_slip_ratio_RL = "0.00".to_string();
            data.tyre_slip_ratio_RR = "0.00".to_string();
        }

        data.time_on_track =
            Duration::from_secs(LittleEndian::read_i32(&ddata[0x80..0x84]) as u64 / 1000);
        data.total_laps = LittleEndian::read_i16(&ddata[0x76..0x78]);
        data.current_position = LittleEndian::read_i16(&ddata[0x84..0x86]);
        data.total_positions = LittleEndian::read_i16(&ddata[0x86..0x88]);
        data.car_id = LittleEndian::read_i32(&ddata[0x124..0x128]);
        data.throttle = (LittleEndian::read_u16(&ddata[0x90..0x92]) as f32 / 2.55) * 100.0;
        data.rpm = LittleEndian::read_f32(&ddata[0x3C..0x40]);
        data.rpm_rev_warning = LittleEndian::read_u16(&ddata[0x88..0x8A]);
        data.brake = (LittleEndian::read_u16(&ddata[0x90..0x92]) as f32 / 2.55) * 100.0;
        data.rpm_rev_limiter = LittleEndian::read_u16(&ddata[0x8A..0x8C]);
        data.estimated_top_speed = LittleEndian::read_i16(&ddata[0x8C..0x8E]);
        data.clutch = LittleEndian::read_f32(&ddata[0xF4..0xF8]);
        data.clutch_engaged = LittleEndian::read_f32(&ddata[0xF8..0xFC]);
        data.rpm_after_clutch = LittleEndian::read_f32(&ddata[0xFC..0x100]);
        data.oil_temp = LittleEndian::read_f32(&ddata[0x5C..0x60]);
        data.water_temp = LittleEndian::read_f32(&ddata[0x58..0x5C]);
        data.oil_pressure = LittleEndian::read_f32(&ddata[0x54..0x58]);
        data.ride_height = 1000.0 * LittleEndian::read_f32(&ddata[0x38..0x3C]);
        data.tyre_temp_FL = LittleEndian::read_f32(&ddata[0x60..0x64]);
        data.tyre_temp_FR = LittleEndian::read_f32(&ddata[0x64..0x68]);
        data.suspension_fl = LittleEndian::read_f32(&ddata[0xC4..0xC8]);
        data.suspension_fr = LittleEndian::read_f32(&ddata[0xC8..0xCC]);
        data.tyre_temp_rl = LittleEndian::read_f32(&ddata[0x68..0x6C]);
        data.tyre_temp_rr = LittleEndian::read_f32(&ddata[0x6C..0x70]);
        data.suspension_rl = LittleEndian::read_f32(&ddata[0xCC..0xD0]);
        data.suspension_rr = LittleEndian::read_f32(&ddata[0xD0..0xD4]);
        data.gear_1 = LittleEndian::read_f32(&ddata[0x104..0x108]);
        data.gear_2 = LittleEndian::read_f32(&ddata[0x108..0x10C]);
        data.gear_3 = LittleEndian::read_f32(&ddata[0x10C..0x110]);
        data.gear_4 = LittleEndian::read_f32(&ddata[0x110..0x114]);
        data.gear_5 = LittleEndian::read_f32(&ddata[0x114..0x118]);
        data.gear_6 = LittleEndian::read_f32(&ddata[0x118..0x11C]);
        data.gear_7 = LittleEndian::read_f32(&ddata[0x11C..0x120]);
        data.gear_8 = LittleEndian::read_f32(&ddata[0x120..0x124]);
        data.position_x = LittleEndian::read_f32(&ddata[0x04..0x08]);
        data.position_y = LittleEndian::read_f32(&ddata[0x08..0x0C]);
        data.position_z = LittleEndian::read_f32(&ddata[0x0C..0x10]);
        data.velocity_x = LittleEndian::read_f32(&ddata[0x10..0x14]);
        data.velocity_y = LittleEndian::read_f32(&ddata[0x14..0x18]);
        data.velocity_z = LittleEndian::read_f32(&ddata[0x18..0x1C]);
        data.rotation_pitch = LittleEndian::read_f32(&ddata[0x1C..0x20]);
        data.rotation_yaw = LittleEndian::read_f32(&ddata[0x20..0x24]);
        data.rotation_roll = LittleEndian::read_f32(&ddata[0x24..0x28]);
        data.angular_velocity_x = LittleEndian::read_f32(&ddata[0x2C..0x30]);
        data.angular_velocity_y = LittleEndian::read_f32(&ddata[0x30..0x34]);
        data.angular_velocity_z = LittleEndian::read_f32(&ddata[0x34..0x38]);

        let flags = LittleEndian::read_u16(&ddata[0x8E..0x90]);
        data.is_paused = (flags & 0b00000010) != 0;
        data.in_race = (flags & 0b00000001) != 0;

        data
    }
}
