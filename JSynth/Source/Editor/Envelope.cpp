
#include "Envelope.h"

EnvelopeUI::EnvelopeUI(JSynthAudioProcessor& p, juce::AudioProcessorEditor* ed) : audioProcessor(p) {
	editor = ed;

	x, y, width, height = 0;

	attackUI.setSliderStyle(juce::Slider::SliderStyle::RotaryHorizontalVerticalDrag);
	attackUI.setRange(5, 2000, 5);
	attackUI.setTextBoxStyle(juce::Slider::NoTextBox, false, 0, 0);
	attackUI.setPopupDisplayEnabled(true, true, editor);
	attackUI.setTextValueSuffix("ms");
	attackUI.setValue(10);

	attackUI.addListener(this);
	editor->addAndMakeVisible(attackUI);


	decayUI.setSliderStyle(juce::Slider::SliderStyle::RotaryHorizontalVerticalDrag);
	decayUI.setRange(5, 2000, 5);
	decayUI.setTextBoxStyle(juce::Slider::NoTextBox, false, 0, 0);
	decayUI.setPopupDisplayEnabled(true, true, editor);
	decayUI.setTextValueSuffix("ms");
	decayUI.setValue(10);

	decayUI.addListener(this);
	editor->addAndMakeVisible(decayUI);


	sustainUI.setSliderStyle(juce::Slider::SliderStyle::RotaryHorizontalVerticalDrag);
	sustainUI.setRange(0, 1.0, 0.01);
	sustainUI.setTextBoxStyle(juce::Slider::NoTextBox, false, 0, 0);
	sustainUI.setPopupDisplayEnabled(true, true, editor);
	sustainUI.setTextValueSuffix("%");
	sustainUI.setValue(1.0);

	sustainUI.addListener(this);
	editor->addAndMakeVisible(sustainUI);


	releaseUI.setSliderStyle(juce::Slider::SliderStyle::RotaryHorizontalVerticalDrag);
	releaseUI.setRange(5, 2000, 5);
	releaseUI.setTextBoxStyle(juce::Slider::NoTextBox, false, 0, 0);
	releaseUI.setPopupDisplayEnabled(true, true, editor);
	releaseUI.setTextValueSuffix("ms");
	releaseUI.setValue(10);

	releaseUI.addListener(this);
	editor->addAndMakeVisible(releaseUI);
}

EnvelopeUI::~EnvelopeUI() {

}

void EnvelopeUI::draw(juce::Graphics& g) {
	g.setColour(juce::Colour::fromRGB(64, 64, 64));
	g.setFont(24);
	g.drawFittedText("Envelope", x + 4, y + 4, width, 24, juce::Justification::left, 1);
}

void EnvelopeUI::resized(int xOffset, int yOffset, int w, int h) {
	x = xOffset;
	y = yOffset;

	width = w;
	height = h;

	attackUI.setBounds(xOffset + 16, yOffset + 32, 48, 48);
	decayUI.setBounds(xOffset + 80, yOffset + 32, 48, 48);
	sustainUI.setBounds(xOffset + 16, yOffset + 96, 48, 48);
	releaseUI.setBounds(xOffset + 80, yOffset + 96, 48, 48);
}


void EnvelopeUI::sliderValueChanged(juce::Slider* slider) {
	float newValue = slider->getValue();

	if (slider == &attackUI) {
		*audioProcessor.attack = newValue;
	} else if (slider == &decayUI) {
		*audioProcessor.decay = newValue;
	} else if (slider == &sustainUI) {
		*audioProcessor.sustain = newValue;
	} else if (slider == &releaseUI) {
		*audioProcessor.release = newValue;
	}
}
