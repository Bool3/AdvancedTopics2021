
#include <cmath>

#include "JVoice.h"

float noteToFrequency(char noteNumber) {
	return 440.0 * std::powf(2.0, (((float)noteNumber - 69.0) / 12.0));
}


JVoice::JVoice() {
	sampleRate = 44100.0;
	note = 0;
	on = false;

	oscillator = new JOsc(noteToFrequency(69), sampleRate);
	envelope = new Adsr();
}

JVoice::~JVoice() {

}

void JVoice::updateSampleRate(float sr) {
	sampleRate = sr;

	oscillator->updateSampleRate(sampleRate);
}

void JVoice::reset() {
	on = false;
	oscillator->reset();
	envelope->reset();
}

void JVoice::play(char n, char velocity) {
	if (velocity != 0) {
		note = n;
		oscillator->updateFrequency(noteToFrequency(note));
		oscillator->reset();
		on = true;
		envelope->start(velocity);
	}
}

void JVoice::stop() {
	on = false;
}

void JVoice::releaseEnvelope() {
	envelope->startRelease();
}

void JVoice::multiplyFrequency(float multiplier) {
	float newFrequency = noteToFrequency(note) * multiplier;

	oscillator->updateFrequency(newFrequency);
}

float JVoice::process(Wave wave) {
	float val = 0.0;

	if (on) {
		val = oscillator->process(wave);
		val = envelope->process(val);

		if (envelope->isDone) {
			stop();
		}
	}

	return val;
}