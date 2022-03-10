
#pragma once

class Adsr {
public:
	Adsr();
	~Adsr();

	void start(char velocity);
	void startRelease();
	void reset();
	float process(float valBefore);


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
