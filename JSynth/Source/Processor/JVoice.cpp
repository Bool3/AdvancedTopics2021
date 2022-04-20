
#include <cmath>

#include "JVoice.h"

float noteToFrequency(char noteNumber) {
	return 440.0 * std::powf(2.0, (((float)noteNumber - 69.0) / 12.0));
}


JVoice::JVoice() {
	sampleRate = 44100.0;
	note = 0;
	on = false;

	osc1 = new JOsc(noteToFrequency(69), sampleRate);
	osc2 = new JOsc(noteToFrequency(69), sampleRate);

	osc1Detune = 1.0;
	osc2Detune = 1.0;

	envelope = new Adsr();
}

JVoice::~JVoice() {

}

void JVoice::updateSampleRate(float sr) {
	sampleRate = sr;

	osc1->updateSampleRate(sampleRate);
	osc2->updateSampleRate(sampleRate);
}

void JVoice::reset() {
	on = false;
	osc1->reset();
	osc2->reset();
	envelope->reset();
}

void JVoice::play(char n, char velocity) {
	if (velocity != 0) {
		note = n;

		osc1->updateFrequency(noteToFrequency(note));
		osc1->reset();

		osc2->updateFrequency(noteToFrequency(note));
		osc2->reset();

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

	osc1->updateFrequency(newFrequency * osc1Detune);
	osc2->updateFrequency(newFrequency * osc2Detune);
}

float JVoice::process(Wave wave1, Wave wave2, float osc1Volume, float osc2Volume) {
	float val = 0.0;

	if (on) {
		val += osc1->process(wave1) * osc1Volume;
		val += osc2->process(wave2) * osc2Volume;

		val = envelope->process(val);

		if (envelope->isDone) {
			stop();
		}
	}

	return val;
}