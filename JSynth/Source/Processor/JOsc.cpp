
#include <math.h>

#include "JOsc.h"

constexpr float PI = 3.14159265358979323846;
constexpr float TWO_PI = (2 * PI);

constexpr float BLEP_MAX = 1.4142135;

float blepDown(float phase) {
	return (cos(phase) - (cos(phase * 3.0) / 3.0)) * (3.0 / 2.0);
}

float blepUp(float phase) {
	return blepDown(phase) * -1.0;
}

JOsc::JOsc(float freq, float sr) {
	sampleRate = sr;
	frequency = freq;

	phase = 0.0;
	phaseIncrement = 0.0;

	blepSliceSampleLength = 4.0;
	blepSliceLength = 0.0;
	lastOutput = 0.0;

	updatePhaseIncrement();
}

JOsc::~JOsc() {

}

void JOsc::reset() {
	phase = 0.0;
}

void JOsc::updateFrequency(float freq) {
	frequency = freq;

	updatePhaseIncrement();
}

void JOsc::updateSampleRate(float sr) {
	sampleRate = sr;

	updatePhaseIncrement();
}

void JOsc::updatePhaseIncrement() {
	float cyclesPerSample = frequency / sampleRate;
	phaseIncrement = cyclesPerSample * TWO_PI;

	updateBlepSliceLength();
}

void JOsc::updateBlepSliceSampleLength(float sampleLength) {
	blepSliceSampleLength = sampleLength;

	updateBlepSliceLength();
}

void JOsc::updateBlepSliceLength() {
	blepSliceLength = phaseIncrement * blepSliceSampleLength;
}

float JOsc::process(Wave wave) {
	float val = 0.0;

	float squareVal = 0.0;
	float triVal = 0.0;

	switch (wave) {
		case Wave::Sine:
			val = sin(phase);
			break;
		case Wave::Triangle:
			squareVal = process(Wave::Square);

			phase -= phaseIncrement;

			triVal = phaseIncrement * squareVal + (1.0 - phaseIncrement) * lastOutput;

			lastOutput = triVal;

			val = triVal;

			break;
		case Wave::Square:
			if (phase <= blepSliceLength) {
				val = blepUp(phase / blepSliceLength * PI);
			}
			else if (phase < PI) {
				val = 1.0;
			}
			else if (phase <= PI + blepSliceLength) {
				val = blepDown((phase - PI) / blepSliceLength * PI);
			}
			else {
				val = -1.0;
			}

			break;
		case Wave::Saw:
			if (phase <= blepSliceLength) {
				val = blepUp(phase / blepSliceLength * PI);
			} else {
				float shifted_phase = phase - blepSliceLength / 2.0;

				val = 1.0 - (shifted_phase / PI);
			}

			break;
		default:
			val = 0.0;

			break;
	}

	incrementPhase();

	return normalize(val, wave);
}

void JOsc::incrementPhase() {
	phase += phaseIncrement;

	if (phase > TWO_PI) {
		phase -= TWO_PI;
	}
}

float JOsc::normalize(float val, Wave wave) {
	switch (wave) {
		case Wave::Sine:
			return val;
		case Wave::Triangle:
			return val;
		case Wave::Square:
			return val / BLEP_MAX;
		case Wave::Saw:
			return val / BLEP_MAX;
		default:
			return val;
	}
}
