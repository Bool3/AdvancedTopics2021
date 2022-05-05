
#pragma once

#include <JuceHeader.h>

#include "../PluginProcessor.h"

class FiltersUI : private juce::ComboBox::Listener, private juce::Slider::Listener {
public:
	FiltersUI(JSynthAudioProcessor&, juce::AudioProcessorEditor* ed);
	~FiltersUI();

	void draw(juce::Graphics& g);
	void resized(int xOffset, int yOffset, int width, int height);

private:
	JSynthAudioProcessor& audioProcessor;
	juce::AudioProcessorEditor* editor;

	int x;
	int y;
	int width;
	int height;

	juce::ComboBox typeUI;
	juce::Slider cutoffUI;
	juce::Slider resonanceUI;

	void comboBoxChanged(juce::ComboBox* comboBox) override;
	void sliderValueChanged(juce::Slider* slider) override;
};

