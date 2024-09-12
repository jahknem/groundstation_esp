/// Calculates the angular position in degrees based on the ADC value relative to a reference value.
///
/// This function computes the angular difference between `adc_value` and `adc_reference`,
/// considering the ADC range defined by `adc_min` and `adc_max`. The result is a floating-point
/// angle in degrees within the range [0, 360).
///
/// # Parameters
///
/// - `adc_min`: The minimum possible ADC value (inclusive).
/// - `adc_max`: The maximum possible ADC value (inclusive).
/// - `adc_value`: The current ADC value to convert to degrees.
/// - `adc_reference`: The reference ADC value representing 0 degrees.
///
/// # Returns
///
/// A `f32` representing the angular position in degrees.
///
/// # Examples
///
/// ```
/// let angle = calculate_degrees(0, 1023, 512, 0);
/// assert_eq!(angle, 180.0);
/// ```
///
/// # Notes
///
/// - The function uses wrapping arithmetic to handle cases where the ADC value wraps around the
///   minimum or maximum limits.
/// - The calculation ensures that the returned angle is always within [0, 360) degrees.
///
/// # Safety
///
/// This function does not panic and is safe to use with any `u16` values for the parameters.
pub fn calculate_degrees(adc_min: u16, adc_max: u16, adc_value: u16, adc_reference: u16) -> f32 {
    let adc_span = adc_max - adc_min + 1;
    let adc_cleaned_var = (adc_value.wrapping_sub(adc_reference).wrapping_add(adc_span)) % adc_span;
    adc_cleaned_var as f32 / (adc_span - 1) as f32 * 360.0
}