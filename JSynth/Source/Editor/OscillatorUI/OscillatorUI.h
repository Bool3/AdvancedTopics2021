
#pragma once

#include <JuceHeader.h>

#include "../../PluginProcessor.h"

enum class OscillatorNumber {
	One = 1,
	Two = 2,
};

class OscillatorUI : private juce::ComboBox::Listener, private juce::Slider::Listener {
public:
	OscillatorUI(JSynthAudioProcessor&, juce::AudioProcessorEditor* ed, OscillatorNumber oscillatorNumber);
	~OscillatorUI();

	void draw(juce::Graphics& g);
	void resized(int xOffset, int yOffset, int width, int height);

private:
	JSynthAudioProcessor& audioProcessor;
	juce::AudioProcessorEditor* editor;

	OscillatorNumber oscillatorNumber;

	int x;
	int y;
	int width;
	int height;

	juce::ComboBox waveUI;
	juce::Slider volumeUI;
	juce::Slider detuneSemitonesUI;
	juce::Slider detuneCentsUI;

	void comboBoxChanged(juce::ComboBox* comboBox) override;
	void sliderValueChanged(juce::Slider* slider) override;
};

