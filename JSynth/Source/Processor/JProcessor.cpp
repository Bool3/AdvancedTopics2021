
#include "JProcessor.h"

#include "../PluginProcessor.h"

#include "JVoice.h"
#include "Route.h"

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

	lfo = new JOsc(0.0, 44100.0);

	pitchBendMultiplier = 1.0;
}

JProcessor::~JProcessor() {

}


void JProcessor::updateSampleRate(float newSampleRate) {
	sampleRate = newSampleRate;

	lfo->updateSampleRate(sampleRate);

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

void JProcessor::updatePitchBendMultiplier(int pitchBend) {
	float bend = ((float)pitchBend) - 8192.0;

	float semitones;

	if (bend > 0.0) {
		semitones = ((float)audioProcessor.pitchBendLimit->get()) * (bend / 8191.0);
	} else if (bend == 0) {
		semitones = 0.0;
	} else {
		semitones = ((float)audioProcessor.pitchBendLimit->get()) * (bend / 8192.0);
	}

	pitchBendMultiplier = std::powf(2.0, semitones / 12.0);
}

float JProcessor::process() {
	float volume = audioProcessor.volume->get();
	Wave wave = (Wave)audioProcessor.wave->getIndex();
	float attack = audioProcessor.attack->get();
	float decay = audioProcessor.decay->get();
	float sustain = audioProcessor.sustain->get();
	float release = audioProcessor.release->get();

	float lfoFrequency = audioProcessor.lfoFrequency->get();
	Wave lfoWave = (Wave)audioProcessor.lfoWave->getIndex();
	float lfoIntensity = audioProcessor.lfoIntensity->get();
	Route lfoRoute = (Route)audioProcessor.lfoRoute->getIndex();

	lfo->updateFrequency(lfoFrequency);
	float lfoVal = lfo->process(lfoWave);

	float lfoFrequencyMultiplier = 1.0;
	float lfoAmplitudeMultiplier = 1.0;

	float lfoValTransformed = 0.0;

	switch (lfoRoute) {
		case Route::None:
			break;
		case Route::Amplitude:
			lfoValTransformed = (lfoVal / 2.0) + 0.5;

			lfoAmplitudeMultiplier = (lfoValTransformed * lfoIntensity) + (1.0 - lfoIntensity);

			break;
		case Route::Frequency:
			lfoFrequencyMultiplier = 1.0 + (lfoVal * lfoIntensity);

			break;
	}

	float val = 0;

	for (int i = 0; i < voices->size(); i++) {
		JVoice* voice = voices->at(i);

		if (voice->envelope->isDone) {
			voice->envelope->attack = msToSamples(attack, sampleRate);
			voice->envelope->decay = msToSamples(decay, sampleRate);
			voice->envelope->sustain = sustain;
			voice->envelope->release = msToSamples(release, sampleRate);
		}

		voice->multiplyFrequency(pitchBendMultiplier * lfoFrequencyMultiplier);

		val += volume * voice->process(wave);
	}

	val *= lfoAmplitudeMultiplier;

	return val;
}

void JProcessor::reset() {
	for (int i = 0; i < voices->size(); i++) {
		voices->at(i)->reset();
	}
}
