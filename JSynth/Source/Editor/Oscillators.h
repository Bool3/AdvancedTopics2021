
#pragma once

#include <JuceHeader.h>

#include "../PluginProcessor.h"

#include "OscillatorUI/OscillatorUI.h"

class OscillatorsUI {
public:
	OscillatorsUI(JSynthAudioProcessor&, juce::AudioProcessorEditor* ed);
	~OscillatorsUI();

	void draw(juce::Graphics& g);
	void resized(int xOffset, int yOffset, int width, int height);

private:
	JSynthAudioProcessor& audioProcessor;
	juce::AudioProcessorEditor* editor;

	int x;
	int y;
	int width;
	int height;

	OscillatorUI* oscillatorUI1;
	OscillatorUI* oscillatorUI2;
};
