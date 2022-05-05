
#pragma once

#include <JuceHeader.h>

#include "../PluginProcessor.h"

class EnvelopeUI : juce::Slider::Listener {
public:
	EnvelopeUI(JSynthAudioProcessor&, juce::AudioProcessorEditor* ed);
	~EnvelopeUI();

	void draw(juce::Graphics& g);
	void resized(int xOffset, int yOffset, int width, int height);

private:
	JSynthAudioProcessor& audioProcessor;
	juce::AudioProcessorEditor* editor;

	int x;
	int y;
	int width;
	int height;

	juce::Slider attackUI;
	juce::Slider decayUI;
	juce::Slider sustainUI;
	juce::Slider releaseUI;

	void sliderValueChanged(juce::Slider* slider) override;
};

