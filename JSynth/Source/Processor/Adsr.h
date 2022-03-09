
#pragma once

class Adsr {
public:
	Adsr();
	~Adsr();

	float attack;
	float decay;
	float sustain;
	float release;

	bool isReleasing;
	bool isDone;
private:
	float peak;
	unsigned int currentSample;
	float volume;
};
