
#include "Oscillators.h"

OscillatorsUI::OscillatorsUI() {

}

OscillatorsUI::~OscillatorsUI() {

}

void OscillatorsUI::draw(juce::Graphics& g) {
	g.setColour(juce::Colour::fromRGB(0, 0, 255));
	g.drawRect(x, y, width, height);
}

void OscillatorsUI::resized(int xOffset, int yOffset, int w, int h) {
	x = xOffset;
	y = yOffset;

	width = w;
	height = h;


}
