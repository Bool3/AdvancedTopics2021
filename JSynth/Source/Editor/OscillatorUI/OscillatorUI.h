
#pragma once

#include <JuceHeader.h>

enum class OscillatorNumber {
	One = 1,
	Two = 2,
};

class OscillatorUI : private juce::ComboBox::Listener {
public:
	OscillatorUI(OscillatorNumber oscillatorNumber);
	~OscillatorUI();

	void draw(juce::Graphics& g);
	void resized(int xOffset, int yOffset, int width, int height);

private:
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
};

