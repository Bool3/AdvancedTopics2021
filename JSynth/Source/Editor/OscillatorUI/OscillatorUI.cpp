
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


	volumeUI.setSliderStyle(juce::Slider::SliderStyle::RotaryHorizontalVerticalDrag);
	volumeUI.setRange(0.0, 1.0, 0.01);
	volumeUI.setTextBoxStyle(juce::Slider::NoTextBox, false, 0, 0);
	volumeUI.setPopupDisplayEnabled(true, true, editor);
	volumeUI.setTextValueSuffix("%");
	volumeUI.setValue(0.125);

	volumeUI.addListener(this);

	editor->addAndMakeVisible(volumeUI);


	detuneSemitonesUI.setSliderStyle(juce::Slider::SliderStyle::RotaryHorizontalVerticalDrag);
	detuneSemitonesUI.setRange(-36, 36, 1);
	detuneSemitonesUI.setTextBoxStyle(juce::Slider::NoTextBox, false, 0, 0);
	detuneSemitonesUI.setPopupDisplayEnabled(true, true, editor);
	detuneSemitonesUI.setValue(0);

	detuneSemitonesUI.addListener(this);

	editor->addAndMakeVisible(detuneSemitonesUI);


	detuneCentsUI.setSliderStyle(juce::Slider::SliderStyle::RotaryHorizontalVerticalDrag);
	detuneCentsUI.setRange(-100, 100, 1);
	detuneCentsUI.setTextBoxStyle(juce::Slider::NoTextBox, false, 0, 0);
	detuneCentsUI.setPopupDisplayEnabled(true, true, editor);
	detuneCentsUI.setValue(0);

	detuneCentsUI.addListener(this);

	editor->addAndMakeVisible(detuneCentsUI);
}

OscillatorUI::~OscillatorUI() {

}

void OscillatorUI::draw(juce::Graphics& g) {
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

	volumeUI.setBounds(xOffset, yOffset + 80, 64, 64);

	detuneSemitonesUI.setBounds(xOffset + 80, yOffset + 80, 64, 64);

	detuneCentsUI.setBounds(xOffset + 160, yOffset + 80, 64, 64);
}

void OscillatorUI::comboBoxChanged(juce::ComboBox* comboBox) {
	int selectedWave = comboBox->getSelectedItemIndex();

	if (oscillatorNumber == OscillatorNumber::One) {
		*audioProcessor.wave1 = selectedWave;
	} else {
		*audioProcessor.wave2 = selectedWave;
	}
}

void OscillatorUI::sliderValueChanged(juce::Slider* slider) {
	float newValue = slider->getValue();

	if (slider == &volumeUI) {
		if (oscillatorNumber == OscillatorNumber::One) {
			*audioProcessor.volume1 = newValue;
		} else {
			*audioProcessor.volume2 = newValue;
		}
	} else if (slider == &detuneSemitonesUI) {
		if (oscillatorNumber == OscillatorNumber::One) {
			*audioProcessor.detuneSemitones1 = newValue;
		} else {
			*audioProcessor.detuneSemitones2 = newValue;
		}
	} else if (slider == &detuneCentsUI) {
		if (oscillatorNumber == OscillatorNumber::One) {
			*audioProcessor.detuneCents1 = newValue;
		} else {
			*audioProcessor.detuneCents2 = newValue;
		}
	}
}
