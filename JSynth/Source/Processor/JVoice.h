

#pragma once

#include "Wave.h"
#include "Adsr.h"

class JVoice {
public:
	JVoice();
	~JVoice();

	void updateSampleRate(float sampleRate);

	void reset();

	void play(char note, char velocity);

	void stop();

	void releaseEnvelope();

	float process(Wave wave);

	char note;
	bool on;

	Adsr envelope;
private:
	float sampleRate;

};
