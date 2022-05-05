
#pragma once

#include <JuceHeader.h>

#include "../PluginProcessor.h"

class LFOUI : private juce::ComboBox::Listener, private juce::Slider::Listener {
public:
	LFOUI(JSynthAudioProcessor&, juce::AudioProcessorEditor* ed);
	~LFOUI();

	void draw(juce::Graphics& g);
	void resized(int xOffset, int yOffset, int width, int height);

private:
	JSynthAudioProcessor& audioProcessor;
	juce::AudioProcessorEditor* editor;

	int x;
	int y;
	int width;
	int height;

	juce::ComboBox routeUI;
	juce::ComboBox waveUI;
	juce::Slider frequencyUI;
	juce::Slider intensityUI;

	void comboBoxChanged(juce::ComboBox* comboBox) override;
	void sliderValueChanged(juce::Slider* slider) override;
};

