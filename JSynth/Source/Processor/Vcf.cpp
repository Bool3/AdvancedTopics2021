
#include<math.h>

#include "Vcf.h"


constexpr float PI = 3.14159265358979323846;
constexpr float TWO_PI = (2 * PI);

Vcf::Vcf() {
	sampleRate = 44100.0;

	previousHighPassOutput = 0.0;
	previousBandPassOutput = 0.0;
	previousLowPassOutput = 0.0;

	previousInput = 0.0;
}

Vcf::~Vcf() {

}

void Vcf::updateSampleRate(float sr) {
	sampleRate = sr;
}

float Vcf::svf(float val, float cutoff, float qFactor, FilterType filterType) {
	float f = sin(PI * cutoff / sampleRate);

	float highPassOutput = val - previousLowPassOutput - (qFactor * previousBandPassOutput);

	float bandPassOutput = (f * highPassOutput) + previousBandPassOutput;

	float lowPassOutput = (f * bandPassOutput) + previousLowPassOutput;
	

	previousHighPassOutput = highPassOutput;
	previousBandPassOutput = bandPassOutput;
	previousLowPassOutput = lowPassOutput;

	previousInput = val;
	

	if (filterType == FilterType::HighPass) {
		return highPassOutput;
	} else if (filterType == FilterType::BandPass) {
		return bandPassOutput;
	} else if (filterType == FilterType::LowPass) {
		return lowPassOutput;
	} else {
		// filtertype is none
	}
}
