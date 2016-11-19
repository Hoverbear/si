use base::*;
use prefix::*;
use {BigInt, BigRational};

// These tests ensure that making something a prefix then taking it back gives you the same thing again.
quickcheck! {
  fn test_yotta_from_meter_and_back(meter: Meter) -> bool {
    Yotta::<Meter>::from(meter.clone()).base() == meter
  }

  fn test_zetta_from_meter_and_back(meter: Meter) -> bool {
    Zetta::<Meter>::from(meter.clone()).base() == meter
  }

  fn test_exa_from_meter_and_back(meter: Meter) -> bool {
    Exa::<Meter>::from(meter.clone()).base() == meter
  }

  fn test_peta_from_meter_and_back(meter: Meter) -> bool {
      Peta::<Meter>::from(meter.clone()).base() == meter
  }

  fn test_tera_from_meter_and_back(meter: Meter) -> bool {
      Tera::<Meter>::from(meter.clone()).base() == meter
  }

  fn test_giga_from_meter_and_back(meter: Meter) -> bool {
      Giga::<Meter>::from(meter.clone()).base() == meter
  }

  fn test_mega_from_meter_and_back(meter: Meter) -> bool {
      Mega::<Meter>::from(meter.clone()).base() == meter
  }

  fn test_kilo_from_meter_and_back(meter: Meter) -> bool {
      Kilo::<Meter>::from(meter.clone()).base() == meter
  }

  fn test_hecto_from_meter_and_back(meter: Meter) -> bool {
    Hecto::<Meter>::from(meter.clone()).base() == meter
  }

  fn test_deka_from_meter_and_back(meter: Meter) -> bool {
      Deka::<Meter>::from(meter.clone()).base() == meter
  }


  fn test_deci_from_meter_and_back(meter: Meter) -> bool {
      Deci::<Meter>::from(meter.clone()).base() == meter
  }

  fn test_centi_from_meter_and_back(meter: Meter) -> bool {
    Centi::<Meter>::from(meter.clone()).base() == meter
  }

  fn test_milli_from_meter_and_back(meter: Meter) -> bool {
    Milli::<Meter>::from(meter.clone()).base() == meter
  }

  fn test_micro_from_meter_and_back(meter: Meter) -> bool {
    Micro::<Meter>::from(meter.clone()).base() == meter
  }

  fn test_nano_from_meter_and_back(meter: Meter) -> bool {
      Nano::<Meter>::from(meter.clone()).base() == meter
  }

  fn test_pico_from_meter_and_back(meter: Meter) -> bool {
      Pico::<Meter>::from(meter.clone()).base() == meter
  }

  fn test_femto_from_meter_and_back(meter: Meter) -> bool {
    Femto::<Meter>::from(meter.clone()).base() == meter
  }

  fn test_atto_from_meter_and_back(meter: Meter) -> bool {
      Atto::<Meter>::from(meter.clone()).base() == meter
  }

  fn test_zepto_from_meter_and_back(meter: Meter) -> bool {
    Zepto::<Meter>::from(meter.clone()).base() == meter
  }

  fn test_yocto_from_meter_and_back(meter: Meter) -> bool {
    Yocto::<Meter>::from(meter.clone()).base() == meter
  }
}