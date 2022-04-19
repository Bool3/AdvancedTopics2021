
#pragma once

#include "FilterType.h"


class Vcf {
public:
	Vcf();
	~Vcf();

	void updateSampleRate(float sampleRate);

	float svf(float val, float cutoff, float qFactor, FilterType filterType);

private:
	float sampleRate;

	float previousHighPassOutput;
	float previousBandPassOutput;
	float previousLowPassOutput;

	float previousInput;

};
