
#include "JProcessor.h"

#include "../PluginProcessor.h"

#include "JVoice.h"

unsigned int msToSamples(float time, float sampleRate) {
	return (unsigned int)(sampleRate * time / 1000.0);
}


JProcessor::JProcessor(JSynthAudioProcessor& p) : audioProcessor(p) {
	sampleRate = 44100.0;

	voices = new std::vector<JVoice*>();

	for (int i = 0; i < 128; i++) {
		JVoice* voice = new JVoice();

		voice->envelope->attack = audioProcessor.attack->get();
		voice->envelope->decay = audioProcessor.decay->get();
		voice->envelope->sustain = audioProcessor.sustain->get();
		voice->envelope->release = audioProcessor.release->get();

		voices->push_back(voice);
	}
}

JProcessor::~JProcessor() {

}


void JProcessor::updateSampleRate(float newSampleRate) {
	sampleRate = newSampleRate;

	for (int i = 0; i < voices->size(); i++) {
		voices->at(i)->updateSampleRate(sampleRate);
	}
}


JVoice* JProcessor::findAvailableVoice() {
	for (int i = 0; i < voices->size(); i++) {
		if (!voices->at(i)->on) {
			return voices->at(i);
		}
	}

	return nullptr;
}

void JProcessor::noteOn(char note, char velocity) {
	JVoice* voice = findAvailableVoice();

	if (voice != nullptr) {
		voice->play(note, velocity);
	}
}

void JProcessor::noteOff(char note) {
	for (int i = 0; i < voices->size(); i++) {
		JVoice* voice = voices->at(i);

		if (voice->note == note && !voice->envelope->isReleasing) {
			voice->releaseEnvelope();
		}
	}
}

float JProcessor::process() {
	float volume = audioProcessor.volume->get();
	Wave wave = (Wave)audioProcessor.wave->getIndex();
	float attack = audioProcessor.attack->get();
	float decay = audioProcessor.decay->get();
	float sustain = audioProcessor.sustain->get();
	float release = audioProcessor.release->get();


	float val = 0;

	for (int i = 0; i < voices->size(); i++) {
		JVoice* voice = voices->at(i);

		if (voice->envelope->isDone) {
			voice->envelope->attack = msToSamples(attack, sampleRate);
			voice->envelope->decay = msToSamples(decay, sampleRate);
			voice->envelope->sustain = sustain;
			voice->envelope->release = msToSamples(release, sampleRate);
		}

		val += volume * voice->process(wave);
	}

	return val;
}

void JProcessor::reset() {
	for (int i = 0; i < voices->size(); i++) {
		voices->at(i)->reset();
	}
}
