

#pragma once

#include "Wave.h"
#include "Adsr.h"
#include "JOsc.h"

class JVoice {
public:
	JVoice();
	~JVoice();

	void updateSampleRate(float sampleRate);

	void reset();

	void play(char note, char velocity);

	void stop();

	void releaseEnvelope();

	void multiplyFrequency(float multiplier);

	float process(Wave wave1, Wave wave2, float osc1Volume, float osc2Volume);

	char note;
	bool on;

	Adsr* envelope;

	float osc1Detune;
	float osc2Detune;
private:
	float sampleRate;

	JOsc* osc1;
	JOsc* osc2;
};
