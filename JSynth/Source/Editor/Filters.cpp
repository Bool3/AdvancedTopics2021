
#include "Filters.h"

FiltersUI::FiltersUI(JSynthAudioProcessor& p, juce::AudioProcessorEditor* ed) : audioProcessor(p) {
	editor = ed;

	x, y, width, height = 0;


	typeUI.addItemList({ "None", "High Pass", "Band Pass", "Low Pass" }, 1);
	typeUI.setSelectedItemIndex(0);

	typeUI.addListener(this);

	editor->addAndMakeVisible(typeUI);


	cutoffUI.setSliderStyle(juce::Slider::SliderStyle::RotaryHorizontalVerticalDrag);
	cutoffUI.setRange(0.0, 22050.0, 5.0);
	cutoffUI.setTextBoxStyle(juce::Slider::NoTextBox, false, 0, 0);
	cutoffUI.setPopupDisplayEnabled(true, true, editor);
	cutoffUI.setValue(0.0);

	cutoffUI.addListener(this);

	editor->addAndMakeVisible(cutoffUI);


	resonanceUI.setSliderStyle(juce::Slider::SliderStyle::RotaryHorizontalVerticalDrag);
	resonanceUI.setRange(0, 1.0, 0.01);
	resonanceUI.setTextBoxStyle(juce::Slider::NoTextBox, false, 0, 0);
	resonanceUI.setPopupDisplayEnabled(true, true, editor);
	resonanceUI.setTextValueSuffix("%");
	resonanceUI.setValue(0.0);

	resonanceUI.addListener(this);
	editor->addAndMakeVisible(resonanceUI);
}

FiltersUI::~FiltersUI() {

}

void FiltersUI::draw(juce::Graphics& g) {
	g.setColour(juce::Colour::fromRGB(64, 64, 64));
	g.setFont(24);
	g.drawFittedText("Filter", x + 4, y + 4, width, 24, juce::Justification::left, 1);
}

void FiltersUI::resized(int xOffset, int yOffset, int w, int h) {
	x = xOffset;
	y = yOffset;

	width = w;
	height = h;

	typeUI.setBounds(xOffset, yOffset + 32, w, 32);

	cutoffUI.setBounds(xOffset, yOffset + 92, 48, 48);
	resonanceUI.setBounds(xOffset + 64, yOffset + 92, 48, 48);
}

void FiltersUI::comboBoxChanged(juce::ComboBox* comboBox) {
	int selectedFilterType = comboBox->getSelectedItemIndex();

	*audioProcessor.filterType = selectedFilterType;
}

void FiltersUI::sliderValueChanged(juce::Slider* slider) {
	float newValue = slider->getValue();

	if (slider == &cutoffUI) {
		*audioProcessor.filterCutoffFrequency = newValue;
	} else if (slider == &resonanceUI) {
		*audioProcessor.filterResonance = newValue;
	}
}
