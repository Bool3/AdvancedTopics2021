
#pragma once

#include "../PluginProcessor.h"
#include "JVoice.h"
#include "Vcf.h"

class JProcessor {
public:
	JProcessor(JSynthAudioProcessor&);
	~JProcessor();

	void updateSampleRate(float newSampleRate);
	JVoice* findAvailableVoice();

	void noteOn(char note, char velocity);
	void noteOff(char note);

	void updatePitchBendMultiplier(int pitchBend);

	float process();

	void reset();

private:
	float sampleRate;
	JSynthAudioProcessor& audioProcessor;
	std::vector<JVoice*>* voices;
	JOsc* lfo;
	Vcf* vcf;
	float pitchBendMultiplier;

};
