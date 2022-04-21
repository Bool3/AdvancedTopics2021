
#include "OscillatorUI.h"

#include "../../Processor/Wave.h"

OscillatorUI::OscillatorUI(JSynthAudioProcessor& p, juce::AudioProcessorEditor* ed, OscillatorNumber oscNum) : audioProcessor(p) {
	editor = ed;

	oscillatorNumber = oscNum;

	x, y, width, height = 0;

	waveUI.addItemList({ "Sine", "Triangle", "Square", "Saw" }, 1);
	waveUI.setSelectedItemIndex(0);

	waveUI.addListener(this);

	editor->addAndMakeVisible(waveUI);
}

OscillatorUI::~OscillatorUI() {

}

void OscillatorUI::draw(juce::Graphics& g) {
	g.setColour(juce::Colour::fromRGB(255, 0, 255));
	g.drawRect(x, y, width, height);

	juce::String oscTitle;

	if (oscillatorNumber == OscillatorNumber::One) {
		oscTitle = "Osc 1";
	} else {
		oscTitle = "Osc 2";
	}

	g.setColour(juce::Colour::fromRGB(64, 64, 64));

	g.setFont(24);

	g.drawFittedText(oscTitle, x + 4, y + 4, width, 24, juce::Justification::left, 1);
}

void OscillatorUI::resized(int xOffset, int yOffset, int w, int h) {
	x = xOffset;
	y = yOffset;

	width = w;
	height = h;

	waveUI.setBounds(xOffset, yOffset + 32, w, 32);
}

void OscillatorUI::comboBoxChanged(juce::ComboBox* comboBox) {
	int selectedWave = comboBox->getSelectedItemIndex();

	if (oscillatorNumber == OscillatorNumber::One) {
		*audioProcessor.wave1 = selectedWave;
	} else {
		*audioProcessor.wave2 = selectedWave;
	}
}
