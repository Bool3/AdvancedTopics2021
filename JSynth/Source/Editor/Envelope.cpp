
#include "Envelope.h"

EnvelopeUI::EnvelopeUI() {
	x, y, width, height = 0;
}

EnvelopeUI::~EnvelopeUI() {

}

void EnvelopeUI::draw(juce::Graphics& g) {
	g.setColour(juce::Colour::fromRGB(255, 0, 0));
	g.drawRect(x, y, width, height);
}

void EnvelopeUI::resized(int xOffset, int yOffset, int w, int h) {
	x = xOffset;
	y = yOffset;

	width = w;
	height = h;

}
