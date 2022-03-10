
#pragma once

#include "Wave.h"


class JOsc {
public:
	JOsc(float frequency, float sampleRate);
	~JOsc();

	void reset();
	void updateFrequency(float frequency);
	void updateSampleRate(float sampleRate);

	void updatePhaseIncrement();

	void updateBlepSliceSampleLength(float sampleLength);
	void updateBlepSliceLength();

	float process(Wave wave);

	void incrementPhase();

	float normalize(float val, Wave wave);

private:
	float sampleRate;
	float frequency;

	float phase;
	float phaseIncrement;

	float blepSliceSampleLength;
	float blepSliceLength;

	float lastOutput;
};