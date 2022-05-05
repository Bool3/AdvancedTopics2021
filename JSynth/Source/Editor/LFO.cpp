
#include "LFO.h"

LFOUI::LFOUI(JSynthAudioProcessor& p, juce::AudioProcessorEditor* ed) : audioProcessor(p) {
	editor = ed;

	x, y, width, height = 0;


	routeUI.addItemList({ "None", "Amplitude", "Frequency" }, 1);
	routeUI.setSelectedItemIndex(0);

	routeUI.addListener(this);

	editor->addAndMakeVisible(routeUI);


	waveUI.addItemList({ "Sine", "Triangle", "Square", "Saw" }, 1);
	waveUI.setSelectedItemIndex(0);

	waveUI.addListener(this);

	editor->addAndMakeVisible(waveUI);


	frequencyUI.setSliderStyle(juce::Slider::SliderStyle::RotaryHorizontalVerticalDrag);
	frequencyUI.setRange(0, 1000, 1);
	frequencyUI.setTextBoxStyle(juce::Slider::NoTextBox, false, 0, 0);
	frequencyUI.setPopupDisplayEnabled(true, true, editor);
	frequencyUI.setTextValueSuffix("hz");
	frequencyUI.setValue(0);

	frequencyUI.addListener(this);
	editor->addAndMakeVisible(frequencyUI);


	intensityUI.setSliderStyle(juce::Slider::SliderStyle::RotaryHorizontalVerticalDrag);
	intensityUI.setRange(0.0, 1.0, 0.01);
	intensityUI.setTextBoxStyle(juce::Slider::NoTextBox, false, 0, 0);
	intensityUI.setPopupDisplayEnabled(true, true, editor);
	intensityUI.setTextValueSuffix("%");
	intensityUI.setValue(0.0);

	intensityUI.addListener(this);
	editor->addAndMakeVisible(intensityUI);
}

LFOUI::~LFOUI() {

}

void LFOUI::draw(juce::Graphics& g) {
	g.setColour(juce::Colour::fromRGB(64, 64, 64));
	g.setFont(24);
	g.drawFittedText("LFO", x + 4, y + 4, width, 24, juce::Justification::left, 1);
}

void LFOUI::resized(int xOffset, int yOffset, int w, int h) {
	x = xOffset;
	y = yOffset;

	width = w;
	height = h;

	routeUI.setBounds(xOffset, yOffset + 32, w, 32);
	waveUI.setBounds(xOffset, yOffset + 64, w, 32);
	frequencyUI.setBounds(xOffset, yOffset + 108, 48, 48);
	intensityUI.setBounds(xOffset + 64, yOffset + 108, 48, 48);
}

void LFOUI::comboBoxChanged(juce::ComboBox* comboBox) {
	if (comboBox == &routeUI) {
		int selectedRoute = comboBox->getSelectedItemIndex();

		*audioProcessor.lfoRoute = selectedRoute;
	} else if (comboBox == &waveUI) {
		int selectedWave = comboBox->getSelectedItemIndex();

		*audioProcessor.lfoWave = selectedWave;
	}
}

void LFOUI::sliderValueChanged(juce::Slider* slider) {
	float newValue = slider->getValue();

	if (slider == &frequencyUI) {
		*audioProcessor.lfoFrequency = newValue;
	} else if (slider == &intensityUI) {
		*audioProcessor.lfoIntensity = newValue;
	}
}
