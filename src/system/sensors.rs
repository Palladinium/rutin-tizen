use std::ffi::CStr;
use std::fmt::{self, Debug, Display, Formatter};
use std::marker::PhantomData;
use std::{panic, ptr};

use libc::{c_char, c_int, c_void};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// Invalid parameter
    InvalidParameter,
    /// Out of memory
    OutOfMemory,
    /// An input/output error occurred when reading value from system
    IoError,
    /// No permission to use the API
    PermissionDenied,
    /// Not supported parameter (Since 3.0)
    NotSupported,
    /// No data available
    NoData,
    /// Sensor doesn't need calibration
    NotNeedCalibration,
    /// Operation failed
    OperationFailed,
    /// The sensor is supported, but currently not available
    NotAvailable,
    /// Unknown error
    Other(c_int),
}

impl Error {
    fn check(i: c_int) -> Result<()> {
        match i {
            rutin_tizen_sys::sensor_error_e_SENSOR_ERROR_NONE => Ok(()),
            rutin_tizen_sys::sensor_error_e_SENSOR_ERROR_INVALID_PARAMETER => {
                Err(Error::InvalidParameter)
            }
            rutin_tizen_sys::sensor_error_e_SENSOR_ERROR_OUT_OF_MEMORY => Err(Error::OutOfMemory),
            rutin_tizen_sys::sensor_error_e_SENSOR_ERROR_IO_ERROR => Err(Error::IoError),
            rutin_tizen_sys::sensor_error_e_SENSOR_ERROR_PERMISSION_DENIED => {
                Err(Error::PermissionDenied)
            }
            rutin_tizen_sys::sensor_error_e_SENSOR_ERROR_NOT_SUPPORTED => Err(Error::NotSupported),
            rutin_tizen_sys::sensor_error_e_SENSOR_ERROR_NO_DATA => Err(Error::NoData),
            rutin_tizen_sys::sensor_error_e_SENSOR_ERROR_NOT_NEED_CALIBRATION => {
                Err(Error::NotNeedCalibration)
            }
            rutin_tizen_sys::sensor_error_e_SENSOR_ERROR_OPERATION_FAILED => {
                Err(Error::OperationFailed)
            }
            rutin_tizen_sys::sensor_error_e_SENSOR_ERROR_NOT_AVAILABLE => Err(Error::NotAvailable),
            _ => Err(Error::Other(i)),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Error::InvalidParameter => f.write_str("Invalid parameter"),
            Error::OutOfMemory => f.write_str("Out of memory"),
            Error::IoError => {
                f.write_str("An input/output error occurred when reading value from system")
            }
            Error::PermissionDenied => f.write_str("No permission to use the API"),
            Error::NotSupported => f.write_str("Not supported parameter (Since 3.0)"),
            Error::NoData => f.write_str("No data available"),
            Error::NotNeedCalibration => f.write_str("Sensor doesn't need calibration"),
            Error::OperationFailed => f.write_str("Operation failed"),
            Error::NotAvailable => {
                f.write_str("The sensor is supported, but currently not available")
            }
            Error::Other(e) => write!(f, "Unknown error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

/// Sensor data accuracy
pub enum Accuracy {
    /// Undefined
    Undefined,
    /// Not accurate
    Bad,
    /// Moderately accurate
    Normal,
    /// Highly accurate
    Good,
    /// Very highly accurate
    VeryGood,
}

impl From<rutin_tizen_sys::sensor_data_accuracy_e> for Accuracy {
    fn from(accuracy: rutin_tizen_sys::sensor_data_accuracy_e) -> Self {
        match accuracy {
            rutin_tizen_sys::sensor_data_accuracy_e_SENSOR_DATA_ACCURACY_UNDEFINED => {
                Self::Undefined
            }
            rutin_tizen_sys::sensor_data_accuracy_e_SENSOR_DATA_ACCURACY_BAD => Self::Bad,
            rutin_tizen_sys::sensor_data_accuracy_e_SENSOR_DATA_ACCURACY_NORMAL => Self::Normal,
            rutin_tizen_sys::sensor_data_accuracy_e_SENSOR_DATA_ACCURACY_GOOD => Self::Good,
            rutin_tizen_sys::sensor_data_accuracy_e_SENSOR_DATA_ACCURACY_VERYGOOD => Self::VeryGood,
            _ => Self::Undefined,
        }
    }
}

pub trait SensorType: private::SensorTypeSealed {
    type Event: FromSensorEvent;

    #[doc(hidden)]
    const VALUE: rutin_tizen_sys::sensor_type_e;

    fn is_supported() -> Result<bool> {
        let mut supported: bool = false;

        let ret = unsafe {
            rutin_tizen_sys::sensor_is_supported(Self::VALUE, &mut supported as *mut bool)
        };

        Error::check(ret)?;
        Ok(supported)
    }
}

#[doc(hidden)]
pub trait FromSensorEvent: private::FromSensorEventSealed {
    fn from_event(data: rutin_tizen_sys::sensor_event_s) -> Self;
}

pub mod types {
    use super::*;

    pub struct Accelerometer;

    impl SensorType for Accelerometer {
        type Event = AccelerometerEvent;

        const VALUE: rutin_tizen_sys::sensor_type_e =
            rutin_tizen_sys::sensor_type_e_SENSOR_ACCELEROMETER;
    }

    impl private::SensorTypeSealed for Accelerometer {}

    pub struct AccelerometerEvent {
        /// Units: microseconds
        pub timestamp: u64,
        /// Units: m/s^2
        pub x: f32,
        /// Units: m/s^2
        pub y: f32,
        /// Units: m/s^2
        pub z: f32,
    }

    impl FromSensorEvent for AccelerometerEvent {
        fn from_event(data: rutin_tizen_sys::sensor_event_s) -> Self {
            Self {
                timestamp: data.timestamp,
                x: data.values[0],
                y: data.values[1],
                z: data.values[2],
            }
        }
    }

    impl private::FromSensorEventSealed for AccelerometerEvent {}

    pub struct Gravity;

    impl SensorType for Gravity {
        type Event = GravityEvent;

        const VALUE: rutin_tizen_sys::sensor_type_e = rutin_tizen_sys::sensor_type_e_SENSOR_GRAVITY;
    }

    impl private::SensorTypeSealed for Gravity {}

    pub struct GravityEvent {
        /// Units: microseconds
        pub timestamp: u64,
        /// Units: m/s^2
        pub x: f32,
        /// Units: m/s^2
        pub y: f32,
        /// Units: m/s^2
        pub z: f32,
    }

    impl FromSensorEvent for GravityEvent {
        fn from_event(data: rutin_tizen_sys::sensor_event_s) -> Self {
            Self {
                timestamp: data.timestamp,
                x: data.values[0],
                y: data.values[1],
                z: data.values[2],
            }
        }
    }

    impl private::FromSensorEventSealed for GravityEvent {}

    pub struct LinearAcceleration;

    impl SensorType for LinearAcceleration {
        type Event = LinearAccelerationEvent;

        const VALUE: rutin_tizen_sys::sensor_type_e =
            rutin_tizen_sys::sensor_type_e_SENSOR_LINEAR_ACCELERATION;
    }

    impl private::SensorTypeSealed for LinearAcceleration {}

    pub struct LinearAccelerationEvent {
        /// Units: microseconds
        pub timestamp: u64,
        /// Units: m/s^2
        pub x: f32,
        /// Units: m/s^2
        pub y: f32,
        /// Units: m/s^2
        pub z: f32,
    }

    impl FromSensorEvent for LinearAccelerationEvent {
        fn from_event(data: rutin_tizen_sys::sensor_event_s) -> Self {
            Self {
                timestamp: data.timestamp,
                x: data.values[0],
                y: data.values[1],
                z: data.values[2],
            }
        }
    }

    impl private::FromSensorEventSealed for LinearAccelerationEvent {}

    pub struct Magnetic;

    impl SensorType for Magnetic {
        type Event = MagneticEvent;

        const VALUE: rutin_tizen_sys::sensor_type_e =
            rutin_tizen_sys::sensor_type_e_SENSOR_MAGNETIC;
    }

    impl private::SensorTypeSealed for Magnetic {}

    pub struct MagneticEvent {
        /// Units: microseconds
        pub timestamp: u64,
        /// Units: µT (microteslas)
        pub x: f32,
        /// Units: µT (microteslas)
        pub y: f32,
        /// Units: µT (microteslas)
        pub z: f32,
    }

    impl FromSensorEvent for MagneticEvent {
        fn from_event(data: rutin_tizen_sys::sensor_event_s) -> Self {
            Self {
                timestamp: data.timestamp,
                x: data.values[0],
                y: data.values[1],
                z: data.values[2],
            }
        }
    }

    impl private::FromSensorEventSealed for MagneticEvent {}

    pub struct RotationVector;

    impl SensorType for RotationVector {
        type Event = RotationVectorEvent;

        const VALUE: rutin_tizen_sys::sensor_type_e =
            rutin_tizen_sys::sensor_type_e_SENSOR_ROTATION_VECTOR;
    }

    impl private::SensorTypeSealed for RotationVector {}

    pub struct RotationVectorEvent {
        /// Units: microseconds
        pub timestamp: u64,
        pub accuracy: Accuracy,
        /// Range: [-1,1]
        pub x: f32,
        /// Range: [-1,1]
        pub y: f32,
        /// Range: [-1,1]
        pub z: f32,
        /// Range: [-1,1]
        pub w: f32,
    }

    impl FromSensorEvent for RotationVectorEvent {
        fn from_event(data: rutin_tizen_sys::sensor_event_s) -> Self {
            Self {
                timestamp: data.timestamp,
                accuracy: Accuracy::from(data.accuracy),
                x: data.values[0],
                y: data.values[1],
                z: data.values[2],
                w: data.values[3],
            }
        }
    }

    impl private::FromSensorEventSealed for RotationVectorEvent {}

    pub struct Orientation;

    impl SensorType for Orientation {
        type Event = OrientationEvent;

        const VALUE: rutin_tizen_sys::sensor_type_e =
            rutin_tizen_sys::sensor_type_e_SENSOR_ORIENTATION;
    }

    impl private::SensorTypeSealed for Orientation {}

    pub struct OrientationEvent {
        /// Units: microseconds
        pub timestamp: u64,
        /// Units: degrees
        pub azimuth: f32,
        /// Units: degrees
        pub pitch: f32,
        /// Units: degrees
        pub roll: f32,
    }

    impl FromSensorEvent for OrientationEvent {
        fn from_event(data: rutin_tizen_sys::sensor_event_s) -> Self {
            Self {
                timestamp: data.timestamp,
                azimuth: data.values[0],
                pitch: data.values[1],
                roll: data.values[2],
            }
        }
    }

    impl private::FromSensorEventSealed for OrientationEvent {}

    pub struct Gyroscope;

    impl SensorType for Gyroscope {
        type Event = GyroscopeEvent;

        const VALUE: rutin_tizen_sys::sensor_type_e =
            rutin_tizen_sys::sensor_type_e_SENSOR_GYROSCOPE;
    }

    impl private::SensorTypeSealed for Gyroscope {}

    pub struct GyroscopeEvent {
        /// Units: microseconds
        pub timestamp: u64,
        /// Units: degrees/s
        pub x: f32,
        /// Units: degrees/s
        pub y: f32,
        /// Units: degrees/s
        pub z: f32,
    }

    impl FromSensorEvent for GyroscopeEvent {
        fn from_event(data: rutin_tizen_sys::sensor_event_s) -> Self {
            Self {
                timestamp: data.timestamp,
                x: data.values[0],
                y: data.values[1],
                z: data.values[2],
            }
        }
    }

    impl private::FromSensorEventSealed for GyroscopeEvent {}

    pub struct Light;

    impl SensorType for Light {
        type Event = LightEvent;

        const VALUE: rutin_tizen_sys::sensor_type_e = rutin_tizen_sys::sensor_type_e_SENSOR_LIGHT;
    }

    impl private::SensorTypeSealed for Light {}

    pub struct LightEvent {
        /// Units: microseconds
        pub timestamp: u64,
        /// Units: Lux
        pub level: f32,
    }

    impl FromSensorEvent for LightEvent {
        fn from_event(data: rutin_tizen_sys::sensor_event_s) -> Self {
            Self {
                timestamp: data.timestamp,
                level: data.values[0],
            }
        }
    }

    impl private::FromSensorEventSealed for LightEvent {}

    pub struct Proximity;

    impl SensorType for Proximity {
        type Event = ProximityEvent;

        const VALUE: rutin_tizen_sys::sensor_type_e =
            rutin_tizen_sys::sensor_type_e_SENSOR_PROXIMITY;
    }

    impl private::SensorTypeSealed for Proximity {}

    pub enum ProximityEvent {
        /// An object is placed near the proximity sensor
        Near,
        /// No object is placed near the proximity sensor
        Far,
        /// Unknown proximity sensor status
        Unknown,
    }

    impl FromSensorEvent for ProximityEvent {
        fn from_event(data: rutin_tizen_sys::sensor_event_s) -> Self {
            match data.values[0] as u32 {
                rutin_tizen_sys::sensor_proximity_e_SENSOR_PROXIMITY_NEAR => Self::Near,
                rutin_tizen_sys::sensor_proximity_e_SENSOR_PROXIMITY_FAR => Self::Far,
                _ => Self::Unknown,
            }
        }
    }

    impl private::FromSensorEventSealed for ProximityEvent {}

    pub struct Pressure;

    impl SensorType for Pressure {
        type Event = PressureEvent;

        const VALUE: rutin_tizen_sys::sensor_type_e =
            rutin_tizen_sys::sensor_type_e_SENSOR_PRESSURE;
    }

    impl private::SensorTypeSealed for Pressure {}

    pub struct PressureEvent {
        /// Units: microseconds
        pub timestamp: u64,
        /// Units: hPa (hectopascals)
        pub pressure: f32,
    }

    impl FromSensorEvent for PressureEvent {
        fn from_event(data: rutin_tizen_sys::sensor_event_s) -> Self {
            Self {
                timestamp: data.timestamp,
                pressure: data.values[0],
            }
        }
    }

    impl private::FromSensorEventSealed for PressureEvent {}

    pub struct Ultraviolet;

    impl SensorType for Ultraviolet {
        type Event = UltravioletEvent;

        const VALUE: rutin_tizen_sys::sensor_type_e =
            rutin_tizen_sys::sensor_type_e_SENSOR_ULTRAVIOLET;
    }

    impl private::SensorTypeSealed for Ultraviolet {}

    pub struct UltravioletEvent {
        /// Units: microseconds
        pub timestamp: u64,
        /// Range: [0,15]
        pub uv_index: f32,
    }

    impl FromSensorEvent for UltravioletEvent {
        fn from_event(data: rutin_tizen_sys::sensor_event_s) -> Self {
            Self {
                timestamp: data.timestamp,
                uv_index: data.values[0],
            }
        }
    }

    impl private::FromSensorEventSealed for UltravioletEvent {}

    pub struct Temperature;

    impl SensorType for Temperature {
        type Event = TemperatureEvent;

        const VALUE: rutin_tizen_sys::sensor_type_e =
            rutin_tizen_sys::sensor_type_e_SENSOR_TEMPERATURE;
    }

    impl private::SensorTypeSealed for Temperature {}

    pub struct TemperatureEvent {
        /// Units: microseconds
        pub timestamp: u64,
        /// Unit: Celsius
        pub temperature: f32,
    }

    impl FromSensorEvent for TemperatureEvent {
        fn from_event(data: rutin_tizen_sys::sensor_event_s) -> Self {
            Self {
                timestamp: data.timestamp,
                temperature: data.values[0],
            }
        }
    }

    impl private::FromSensorEventSealed for TemperatureEvent {}

    pub struct Humidity;

    impl SensorType for Humidity {
        type Event = HumidityEvent;

        const VALUE: rutin_tizen_sys::sensor_type_e =
            rutin_tizen_sys::sensor_type_e_SENSOR_HUMIDITY;
    }

    impl private::SensorTypeSealed for Humidity {}

    pub struct HumidityEvent {
        /// Units: microseconds
        pub timestamp: u64,
        /// Unit: Percent
        pub humidity: f32,
    }

    impl FromSensorEvent for HumidityEvent {
        fn from_event(data: rutin_tizen_sys::sensor_event_s) -> Self {
            Self {
                timestamp: data.timestamp,
                humidity: data.values[0],
            }
        }
    }

    impl private::FromSensorEventSealed for HumidityEvent {}

    pub struct HeartRateMonitor;

    impl SensorType for HeartRateMonitor {
        type Event = HeartRateMonitorEvent;

        const VALUE: rutin_tizen_sys::sensor_type_e = rutin_tizen_sys::sensor_type_e_SENSOR_HRM;
    }

    impl private::SensorTypeSealed for HeartRateMonitor {}

    pub struct HeartRateMonitorEvent {
        /// Units: microseconds
        pub timestamp: u64,
        /// Unit: BPM (beats per minute)
        pub bpm: f32,
    }

    impl FromSensorEvent for HeartRateMonitorEvent {
        fn from_event(data: rutin_tizen_sys::sensor_event_s) -> Self {
            Self {
                timestamp: data.timestamp,
                bpm: data.values[0],
            }
        }
    }

    impl private::FromSensorEventSealed for HeartRateMonitorEvent {}

    pub struct HeartRateMonitorGreenLed;

    impl SensorType for HeartRateMonitorGreenLed {
        type Event = HeartRateMonitorGreenLedEvent;

        const VALUE: rutin_tizen_sys::sensor_type_e =
            rutin_tizen_sys::sensor_type_e_SENSOR_HRM_LED_GREEN;
    }

    impl private::SensorTypeSealed for HeartRateMonitorGreenLed {}

    pub struct HeartRateMonitorGreenLedEvent {
        /// Units: microseconds
        pub timestamp: u64,
        /// Range:  [0, 4194304]
        pub green_light: f32,
    }

    impl FromSensorEvent for HeartRateMonitorGreenLedEvent {
        fn from_event(data: rutin_tizen_sys::sensor_event_s) -> Self {
            Self {
                timestamp: data.timestamp,
                green_light: data.values[0],
            }
        }
    }

    impl private::FromSensorEventSealed for HeartRateMonitorGreenLedEvent {}

    pub struct HeartRateMonitorRedLed;

    impl SensorType for HeartRateMonitorRedLed {
        type Event = HeartRateMonitorRedLedEvent;

        const VALUE: rutin_tizen_sys::sensor_type_e =
            rutin_tizen_sys::sensor_type_e_SENSOR_HRM_LED_RED;
    }

    impl private::SensorTypeSealed for HeartRateMonitorRedLed {}

    pub struct HeartRateMonitorRedLedEvent {
        /// Units: microseconds
        pub timestamp: u64,
        /// Range:  [0, 4194304]
        pub red_light: f32,
    }

    impl FromSensorEvent for HeartRateMonitorRedLedEvent {
        fn from_event(data: rutin_tizen_sys::sensor_event_s) -> Self {
            Self {
                timestamp: data.timestamp,
                red_light: data.values[0],
            }
        }
    }

    impl private::FromSensorEventSealed for HeartRateMonitorRedLedEvent {}

    pub struct HeartRateMonitorInfraredLed;

    impl SensorType for HeartRateMonitorInfraredLed {
        type Event = HeartRateMonitorInfraredLedEvent;

        const VALUE: rutin_tizen_sys::sensor_type_e =
            rutin_tizen_sys::sensor_type_e_SENSOR_HRM_LED_IR;
    }

    impl private::SensorTypeSealed for HeartRateMonitorInfraredLed {}

    pub struct HeartRateMonitorInfraredLedEvent {
        /// Units: microseconds
        pub timestamp: u64,
        /// Range:  [0, 4194304]
        pub infrared_light: f32,
    }

    impl FromSensorEvent for HeartRateMonitorInfraredLedEvent {
        fn from_event(data: rutin_tizen_sys::sensor_event_s) -> Self {
            Self {
                timestamp: data.timestamp,
                infrared_light: data.values[0],
            }
        }
    }

    impl private::FromSensorEventSealed for HeartRateMonitorInfraredLedEvent {}

    pub struct UncalibratedGyroscope;

    impl SensorType for UncalibratedGyroscope {
        type Event = UncalibratedGyroscopeEvent;

        const VALUE: rutin_tizen_sys::sensor_type_e =
            rutin_tizen_sys::sensor_type_e_SENSOR_GYROSCOPE_UNCALIBRATED;
    }

    impl private::SensorTypeSealed for UncalibratedGyroscope {}

    pub struct UncalibratedGyroscopeEvent {
        /// Units: microseconds
        pub timestamp: u64,
        /// Units: degrees/s
        pub x: f32,
        /// Units: degrees/s
        pub y: f32,
        /// Units: degrees/s
        pub z: f32,
        /// Units: degrees/s
        pub x_drift: f32,
        /// Units: degrees/s
        pub y_drift: f32,
        /// Units: degrees/s
        pub z_drift: f32,
    }

    impl FromSensorEvent for UncalibratedGyroscopeEvent {
        fn from_event(data: rutin_tizen_sys::sensor_event_s) -> Self {
            Self {
                timestamp: data.timestamp,
                x: data.values[0],
                y: data.values[1],
                z: data.values[2],
                x_drift: data.values[3],
                y_drift: data.values[4],
                z_drift: data.values[5],
            }
        }
    }

    impl private::FromSensorEventSealed for UncalibratedGyroscopeEvent {}

    pub struct UncalibratedMagnetic;

    impl SensorType for UncalibratedMagnetic {
        type Event = UncalibratedMagneticEvent;

        const VALUE: rutin_tizen_sys::sensor_type_e =
            rutin_tizen_sys::sensor_type_e_SENSOR_GEOMAGNETIC_UNCALIBRATED;
    }

    impl private::SensorTypeSealed for UncalibratedMagnetic {}

    pub struct UncalibratedMagneticEvent {
        /// Units: microseconds
        pub timestamp: u64,
        /// Units: µT (microteslas)
        pub x: f32,
        /// Units: µT (microteslas)
        pub y: f32,
        /// Units: µT (microteslas)
        pub z: f32,
        /// Units: µT (microteslas)
        pub x_bias: f32,
        /// Units: µT (microteslas)
        pub y_bias: f32,
        /// Units: µT (microteslas)
        pub z_bias: f32,
    }

    impl FromSensorEvent for UncalibratedMagneticEvent {
        fn from_event(data: rutin_tizen_sys::sensor_event_s) -> Self {
            Self {
                timestamp: data.timestamp,
                x: data.values[0],
                y: data.values[1],
                z: data.values[2],
                x_bias: data.values[3],
                y_bias: data.values[4],
                z_bias: data.values[5],
            }
        }
    }

    impl private::FromSensorEventSealed for UncalibratedMagneticEvent {}

    pub struct GyroscopeRotationVector;

    impl SensorType for GyroscopeRotationVector {
        type Event = GyroscopeRotationVectorEvent;

        const VALUE: rutin_tizen_sys::sensor_type_e =
            rutin_tizen_sys::sensor_type_e_SENSOR_GYROSCOPE_ROTATION_VECTOR;
    }

    impl private::SensorTypeSealed for GyroscopeRotationVector {}

    pub struct GyroscopeRotationVectorEvent {
        /// Units: microseconds
        pub timestamp: u64,
        pub accuracy: Accuracy,
        /// Range: [-1,1]
        pub x: f32,
        /// Range: [-1,1]
        pub y: f32,
        /// Range: [-1,1]
        pub z: f32,
        /// Range: [-1,1]
        pub w: f32,
    }

    impl FromSensorEvent for GyroscopeRotationVectorEvent {
        fn from_event(data: rutin_tizen_sys::sensor_event_s) -> Self {
            Self {
                timestamp: data.timestamp,
                accuracy: Accuracy::from(data.accuracy),
                x: data.values[0],
                y: data.values[1],
                z: data.values[2],
                w: data.values[3],
            }
        }
    }

    impl private::FromSensorEventSealed for GyroscopeRotationVectorEvent {}

    pub struct GeomagneticRotationVector;

    impl SensorType for GeomagneticRotationVector {
        type Event = GeomagneticRotationVectorEvent;

        const VALUE: rutin_tizen_sys::sensor_type_e =
            rutin_tizen_sys::sensor_type_e_SENSOR_GEOMAGNETIC_ROTATION_VECTOR;
    }

    impl private::SensorTypeSealed for GeomagneticRotationVector {}

    pub struct GeomagneticRotationVectorEvent {
        /// Units: microseconds
        pub timestamp: u64,
        pub accuracy: Accuracy,
        /// Range: [-1,1]
        pub x: f32,
        /// Range: [-1,1]
        pub y: f32,
        /// Range: [-1,1]
        pub z: f32,
        /// Range: [-1,1]
        pub w: f32,
    }

    impl FromSensorEvent for GeomagneticRotationVectorEvent {
        fn from_event(data: rutin_tizen_sys::sensor_event_s) -> Self {
            Self {
                timestamp: data.timestamp,
                accuracy: Accuracy::from(data.accuracy),
                x: data.values[0],
                y: data.values[1],
                z: data.values[2],
                w: data.values[3],
            }
        }
    }

    impl private::FromSensorEventSealed for GeomagneticRotationVectorEvent {}

    pub struct SignificantMotion;

    impl SensorType for SignificantMotion {
        type Event = SignificantMotionEvent;

        const VALUE: rutin_tizen_sys::sensor_type_e =
            rutin_tizen_sys::sensor_type_e_SENSOR_SIGNIFICANT_MOTION;
    }

    impl private::SensorTypeSealed for SignificantMotion {}

    pub struct SignificantMotionEvent {
        /// Units: microseconds
        pub timestamp: u64,
        /// Undocumented
        pub value: f32,
    }

    impl FromSensorEvent for SignificantMotionEvent {
        fn from_event(data: rutin_tizen_sys::sensor_event_s) -> Self {
            Self {
                timestamp: data.timestamp,
                value: data.values[0],
            }
        }
    }

    impl private::FromSensorEventSealed for SignificantMotionEvent {}

    pub struct HeartRateMonitorBatch;

    impl SensorType for HeartRateMonitorBatch {
        type Event = HeartRateMonitorBatchEvent;

        const VALUE: rutin_tizen_sys::sensor_type_e =
            rutin_tizen_sys::sensor_type_e_SENSOR_HRM_BATCH;
    }

    impl private::SensorTypeSealed for HeartRateMonitorBatch {}

    pub enum HeartRateMonitorBatchState {
        /// Flush but there was no batched data
        NoDataFlush,
        /// Very low measurement reliability
        VeryLowReliability,
        /// Low measurement reliability
        LowReliability,
        /// Device detachment was detected during auto measurement
        DetachedAuto,
        /// Device detachment was detected
        Detached,
        /// The Movement was detected during on-demand measurement
        DetectMove,
        /// Device attachment was detected
        Attached,
        /// Initial state before measurement
        None,
        /// Heart-rate was measured normally
        Ok,
        /// Unknown state
        Unknown(f32),
    }

    impl From<f32> for HeartRateMonitorBatchState {
        fn from(value: f32) -> Self {
            match value as i32 {
                rutin_tizen_sys::sensor_hrm_batch_state_e_SENSOR_HRM_BATCH_STATE_NODATA_FLUSH => Self::NoDataFlush,
                rutin_tizen_sys::sensor_hrm_batch_state_e_SENSOR_HRM_BATCH_STATE_VERYLOW_RELIABILITY => Self::VeryLowReliability,
                rutin_tizen_sys::sensor_hrm_batch_state_e_SENSOR_HRM_BATCH_STATE_LOW_RELIABILITY => Self::LowReliability,
                rutin_tizen_sys::sensor_hrm_batch_state_e_SENSOR_HRM_BATCH_STATE_DETACHED_AUTO => Self::DetachedAuto,
                rutin_tizen_sys::sensor_hrm_batch_state_e_SENSOR_HRM_BATCH_STATE_DETACHED  => Self::Detached,
                rutin_tizen_sys::sensor_hrm_batch_state_e_SENSOR_HRM_BATCH_STATE_DETECT_MOVE => Self::DetectMove,
                rutin_tizen_sys::sensor_hrm_batch_state_e_SENSOR_HRM_BATCH_STATE_ATTACHED => Self::Attached,
                rutin_tizen_sys::sensor_hrm_batch_state_e_SENSOR_HRM_BATCH_STATE_NONE => Self::None,
                rutin_tizen_sys::sensor_hrm_batch_state_e_SENSOR_HRM_BATCH_STATE_OK => Self::Ok,
                _ => Self::Unknown(value),
            }
        }
    }

    pub struct HeartRateMonitorBatchEvent {
        /// Units: microseconds
        pub timestamp: u64,
        pub state: HeartRateMonitorBatchState,
        /// Unit: BPM (beats per minute)
        pub bpm: f32,
        /// R-wave-to-R-wave interval
        /// Unit: milliseconts
        pub r_to_r_interval: f32,
    }

    impl FromSensorEvent for HeartRateMonitorBatchEvent {
        fn from_event(data: rutin_tizen_sys::sensor_event_s) -> Self {
            Self {
                timestamp: data.timestamp,
                state: HeartRateMonitorBatchState::from(data.values[0]),
                bpm: data.values[1],
                r_to_r_interval: data.values[2],
            }
        }
    }

    impl private::FromSensorEventSealed for HeartRateMonitorBatchEvent {}

    pub struct HeartRateMonitorGreenLedBatch;

    impl SensorType for HeartRateMonitorGreenLedBatch {
        type Event = HeartRateMonitorGreenLedBatchEvent;

        const VALUE: rutin_tizen_sys::sensor_type_e =
            rutin_tizen_sys::sensor_type_e_SENSOR_HRM_LED_GREEN_BATCH;
    }

    impl private::SensorTypeSealed for HeartRateMonitorGreenLedBatch {}

    pub struct HeartRateMonitorGreenLedBatchEvent {
        /// Units: microseconds
        pub timestamp: u64,
        /// Range: [0, 4194304]
        pub green_light: f32,
        /// Range: [0, 4096]
        pub x: f32,
        /// Range: [0, 4096]
        pub y: f32,
        /// Range: [0, 4096]
        pub z: f32,
        pub index: usize,
    }

    impl FromSensorEvent for HeartRateMonitorGreenLedBatchEvent {
        fn from_event(data: rutin_tizen_sys::sensor_event_s) -> Self {
            Self {
                timestamp: data.timestamp,
                green_light: data.values[0],
                x: data.values[1],
                y: data.values[2],
                z: data.values[3],
                index: data.values[4] as usize,
            }
        }
    }

    impl private::FromSensorEventSealed for HeartRateMonitorGreenLedBatchEvent {}

    pub struct Pedometer;

    impl SensorType for Pedometer {
        type Event = PedometerEvent;

        const VALUE: rutin_tizen_sys::sensor_type_e =
            rutin_tizen_sys::sensor_type_e_SENSOR_HUMAN_PEDOMETER;
    }

    impl private::SensorTypeSealed for Pedometer {}

    pub enum PedometerState {
        /// Uncertain
        Unknown,
        /// The user is not moving
        Stop,
        /// The user is walking
        Walk,
        /// The user is running
        Run,
    }

    impl From<f32> for PedometerState {
        fn from(value: f32) -> Self {
            match value as i32 {
                rutin_tizen_sys::sensor_pedometer_state_e_SENSOR_PEDOMETER_STATE_UNKNOWN => {
                    Self::Unknown
                }
                rutin_tizen_sys::sensor_pedometer_state_e_SENSOR_PEDOMETER_STATE_STOP => Self::Stop,
                rutin_tizen_sys::sensor_pedometer_state_e_SENSOR_PEDOMETER_STATE_WALK => Self::Walk,
                rutin_tizen_sys::sensor_pedometer_state_e_SENSOR_PEDOMETER_STATE_RUN => Self::Run,
                _ => Self::Unknown,
            }
        }
    }

    pub struct PedometerEvent {
        /// Units: microseconds
        pub timestamp: u64,
        /// Units: steps
        pub steps: u32,
        /// Units: steps
        pub walking_steps: u32,
        /// Units: steps
        pub running_steps: u32,
        /// Units: meters
        pub moving_distance: f32,
        /// Units: kcal
        pub calories_burned: f32,
        /// Units: km/h
        pub last_speed: f32,
        /// Units: steps/s
        pub last_stepping_frequency: f32,
        pub last_state: PedometerState,
    }

    impl FromSensorEvent for PedometerEvent {
        fn from_event(data: rutin_tizen_sys::sensor_event_s) -> Self {
            Self {
                timestamp: data.timestamp,
                steps: data.values[0] as u32,
                walking_steps: data.values[1] as u32,
                running_steps: data.values[2] as u32,
                moving_distance: data.values[3],
                calories_burned: data.values[4],
                last_speed: data.values[5],
                last_stepping_frequency: data.values[6],
                last_state: PedometerState::from(data.values[7]),
            }
        }
    }

    impl private::FromSensorEventSealed for PedometerEvent {}

    pub struct SleepMonitor;

    impl SensorType for SleepMonitor {
        type Event = SleepMonitorEvent;

        const VALUE: rutin_tizen_sys::sensor_type_e =
            rutin_tizen_sys::sensor_type_e_SENSOR_HUMAN_SLEEP_MONITOR;
    }

    impl private::SensorTypeSealed for SleepMonitor {}

    pub enum SleepMonitorState {
        /// Uncertain
        Unknown,
        /// The user is awake
        Wake,
        /// The user is asleep
        Sleep,
    }

    impl From<f32> for SleepMonitorState {
        fn from(value: f32) -> Self {
            match value as i32 {
                rutin_tizen_sys::sensor_sleep_state_e_SENSOR_SLEEP_STATE_UNKNOWN => Self::Unknown,
                rutin_tizen_sys::sensor_sleep_state_e_SENSOR_SLEEP_STATE_WAKE => Self::Wake,
                rutin_tizen_sys::sensor_sleep_state_e_SENSOR_SLEEP_STATE_SLEEP => Self::Sleep,
                _ => Self::Unknown,
            }
        }
    }

    pub struct SleepMonitorEvent {
        /// Units: microseconds
        pub timestamp: u64,
        pub last_state: SleepMonitorState,
    }

    impl FromSensorEvent for SleepMonitorEvent {
        fn from_event(data: rutin_tizen_sys::sensor_event_s) -> Self {
            Self {
                timestamp: data.timestamp,
                last_state: SleepMonitorState::from(data.values[0]),
            }
        }
    }

    impl private::FromSensorEventSealed for SleepMonitorEvent {}
}

pub struct Sensor<T> {
    handle: rutin_tizen_sys::sensor_h,
    _marker: PhantomData<*const T>,
}

impl<T: SensorType> Sensor<T> {
    pub fn get_default() -> Result<Self> {
        let mut handle: rutin_tizen_sys::sensor_h = ptr::null_mut();
        let ret =
            unsafe { rutin_tizen_sys::sensor_get_default_sensor(T::VALUE, &mut handle as *mut _) };

        Error::check(ret)?;
        Ok(Self {
            handle,
            _marker: PhantomData,
        })
    }

    pub fn get_list() -> Result<Vec<Self>> {
        let mut list_ptr: *mut rutin_tizen_sys::sensor_h = ptr::null_mut();
        let mut sensor_count: c_int = 0;

        let ret = unsafe {
            rutin_tizen_sys::sensor_get_sensor_list(
                T::VALUE,
                &mut list_ptr as *mut _,
                &mut sensor_count as *mut c_int,
            )
        };

        Error::check(ret)?;

        unsafe {
            let list: Vec<_> = (0..sensor_count as isize)
                .map(|i| Self {
                    handle: *list_ptr.offset(i),
                    _marker: PhantomData,
                })
                .collect();

            libc::free(list_ptr as *mut c_void);

            Ok(list)
        }
    }

    pub fn name(&self) -> Result<String> {
        let mut name_ptr = ptr::null_mut();

        let ret = unsafe {
            rutin_tizen_sys::sensor_get_name(self.handle, &mut name_ptr as *mut *mut c_char)
        };

        Error::check(ret)?;

        unsafe {
            let name = CStr::from_ptr(name_ptr).to_str().unwrap().to_owned();
            libc::free(name_ptr as *mut c_void);
            Ok(name)
        }
    }

    pub fn is_wake_up(&self) -> Result<bool> {
        let mut is_wake_up: bool = false;

        let ret = unsafe {
            rutin_tizen_sys::sensor_is_wake_up(self.handle, &mut is_wake_up as *mut bool)
        };

        Error::check(ret)?;
        Ok(is_wake_up)
    }
}

impl<T: SensorType> Clone for Sensor<T> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle,
            _marker: PhantomData,
        }
    }
}

impl<T: SensorType> Copy for Sensor<T> {}

pub trait SensorEventHandler<T>
where
    T: SensorType,
{
    fn event(&mut self, event: T::Event);
}

pub struct SensorListenerError<U> {
    pub error: Error,
    pub handler: U,
}

impl<U> Debug for SensorListenerError<U> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("SensorListenerError")
            .field("error", &self.error)
            .finish_non_exhaustive()
    }
}

impl<U> Display for SensorListenerError<U> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", &self.error)
    }
}

impl<U> std::error::Error for SensorListenerError<U> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

struct SensorListenerHandle(Option<rutin_tizen_sys::sensor_listener_h>);

impl SensorListenerHandle {
    fn destroy(&mut self) -> Result<()> {
        if let Some(handle) = self.0.take() {
            Error::check(unsafe { rutin_tizen_sys::sensor_destroy_listener(handle) })?;
        }

        Ok(())
    }
}

impl Drop for SensorListenerHandle {
    fn drop(&mut self) {
        let _ = self.destroy();
    }
}

/// A registered listener for a sensor
pub struct SensorListener<T, U> {
    sensor: Sensor<T>,
    handler: Box<U>,
    handle: SensorListenerHandle,
}

impl<T, U> SensorListener<T, U>
where
    T: SensorType,
    U: SensorEventHandler<T>,
{
    /// Create a new listener that runs the provided handler with sensor data.
    /// Note that the listener will be created stopped, and you need to call [`SensorListener::start`] to start receiving events.
    pub fn new(sensor: Sensor<T>, handler: U) -> std::result::Result<Self, SensorListenerError<U>> {
        let mut handle: rutin_tizen_sys::sensor_listener_h = ptr::null_mut();

        let ret = unsafe {
            rutin_tizen_sys::sensor_create_listener(sensor.handle, &mut handle as *mut _)
        };

        if let Err(error) = Error::check(ret) {
            return Err(SensorListenerError { error, handler });
        }

        let mut self_ = Self {
            sensor,
            handler: Box::new(handler),
            handle: SensorListenerHandle(Some(handle)),
        };

        let ret = unsafe {
            rutin_tizen_sys::sensor_listener_set_events_cb(
                handle,
                Some(sensor_listener_handler::<T, U>),
                self_.handler.as_mut() as *mut _ as *mut c_void,
            )
        };

        if let Err(error) = Error::check(ret) {
            return Err(SensorListenerError {
                error,
                handler: self_.destroy().unwrap_or_else(|e| e.handler),
            });
        }

        Ok(self_)
    }

    /// Returns the associated sensor
    pub fn sensor(&self) -> Sensor<T> {
        self.sensor
    }

    /// Start receiving sensor events.
    pub fn start(&mut self) -> Result<()> {
        let ret = unsafe {
            rutin_tizen_sys::sensor_listener_start(
                *self.handle.0.as_ref().expect("No sensor listener handle"),
            )
        };

        Error::check(ret)
    }

    /// Stop receiving sensor events.
    pub fn stop(&mut self) -> Result<()> {
        let ret = unsafe {
            rutin_tizen_sys::sensor_listener_stop(
                *self.handle.0.as_ref().expect("No sensor listener handle"),
            )
        };

        Error::check(ret)
    }

    /// Destroy this listener and return the underlying handler.
    /// This is automatically called by the `Drop` impl, but you should use this method if you
    /// want to retain the handler or handle any errors that occur during destruction.
    pub fn destroy(mut self) -> std::result::Result<U, SensorListenerError<U>> {
        match self.handle.destroy() {
            Ok(()) => Ok(*self.handler),
            Err(error) => Err(SensorListenerError {
                handler: *self.handler,
                error,
            }),
        }
    }
}

extern "C" fn sensor_listener_handler<T: SensorType, U: SensorEventHandler<T>>(
    _sensor: rutin_tizen_sys::sensor_h,
    events: *mut rutin_tizen_sys::sensor_event_s,
    event_count: c_int,
    data: *mut c_void,
) {
    for i in 0..event_count as isize {
        let event = unsafe { *events.offset(i) };

        let _ = panic::catch_unwind(|| {
            let handler = unsafe { &mut *(data as *mut U) };
            handler.event(T::Event::from_event(event));
        });
    }
}

mod private {
    pub trait SensorTypeSealed {}
    pub trait FromSensorEventSealed {}
}
