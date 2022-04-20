
#include "OscillatorUI.h"

OscillatorUI::OscillatorUI(OscillatorNumber oscNum) {
	oscillatorNumber = oscNum;

	x, y, width, height = 0;

	
}

OscillatorUI::~OscillatorUI() {

}

void OscillatorUI::draw(juce::Graphics& g) {
	g.setColour(juce::Colour::fromRGB(255, 0, 255));
	g.drawRect(x, y, width, height);
}

void OscillatorUI::resized(int xOffset, int yOffset, int w, int h) {
	x = xOffset;
	y = yOffset;

	width = w;
	height = h;

}
