
#include "LFO.h"

LFOUI::LFOUI() {
	x, y, width, height = 0;
}

LFOUI::~LFOUI() {

}

void LFOUI::draw(juce::Graphics& g) {
	g.setColour(juce::Colour::fromRGB(255, 0, 255));
	g.drawRect(x, y, width, height);
}

void LFOUI::resized(int xOffset, int yOffset, int w, int h) {
	x = xOffset;
	y = yOffset;

	width = w;
	height = h;

}
