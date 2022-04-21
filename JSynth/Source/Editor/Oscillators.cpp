
#include "Oscillators.h"

OscillatorsUI::OscillatorsUI(JSynthAudioProcessor& p, juce::AudioProcessorEditor* ed) : audioProcessor(p) {
	editor = ed;

	x, y, width, height = 0;

	oscillatorUI1 = new OscillatorUI(p, editor, OscillatorNumber::One);
	oscillatorUI2 = new OscillatorUI(p, editor, OscillatorNumber::Two);
}

OscillatorsUI::~OscillatorsUI() {

}

void OscillatorsUI::draw(juce::Graphics& g) {
	oscillatorUI1->draw(g);
	oscillatorUI2->draw(g);
}

void OscillatorsUI::resized(int xOffset, int yOffset, int w, int h) {
	x = xOffset;
	y = yOffset;

	width = w;
	height = h;

	oscillatorUI1->resized(xOffset, yOffset, width / 2, height);
	oscillatorUI2->resized(xOffset + (width / 2), yOffset, width / 2, height);
}
