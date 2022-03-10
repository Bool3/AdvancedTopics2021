
#include "Adsr.h"

float line(unsigned int index, float start, float finish, unsigned int duration) {
	return ((float)index) * (finish - start) / ((float)duration) + start;
}

Adsr::Adsr() {
	peak = 0.0;
	sustain = 0.5;

	attack = 44100;
	decay = 44100;
	release = 44100;

	currentSample = 0;
	volume = 0.0;

	isReleasing = false;
	isDone = true;
}

Adsr::~Adsr() {

}

void Adsr::start(char velocity) {
	peak = ((float)velocity) / 127.0;

	currentSample = 0;
	volume = 0;

	isReleasing = false;
	isDone = false;
}

void Adsr::startRelease() {
	isReleasing = true;
	currentSample = attack + decay + 1;
}

void Adsr::reset() {
	peak = 0;
	currentSample = 0;
	volume = 0;
	isReleasing = false;
	isDone = true;
}

float Adsr::process(float valBefore) {
	float val = valBefore;

	if (peak == 0) {
		isDone = true;
		val = 0;
	} else {
		if (!isReleasing) {
			if (currentSample <= attack) {
				volume = line(currentSample, 0, peak, attack);
				currentSample += 1;
			} else if (currentSample <= attack + decay) {
				volume = line(currentSample - attack, peak, sustain * peak, decay);
				currentSample += 1;
			} else {
				volume = sustain * peak;
			}

			val *= volume;
		} else {
			if (currentSample <= attack + decay + release) {
				val *= line(currentSample - (attack + decay), volume, 0, release);
				currentSample += 1;
			} else {
				isDone = true;
				val = 0;
			}
		}
	}

	return val;
}
