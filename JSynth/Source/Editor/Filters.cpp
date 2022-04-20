
#include "Filters.h"

FiltersUI::FiltersUI() {
	x, y, width, height = 0;
}

FiltersUI::~FiltersUI() {

}

void FiltersUI::draw(juce::Graphics& g) {
	g.setColour(juce::Colour::fromRGB(0, 255, 0));
	g.drawRect(x, y, width, height);
}

void FiltersUI::resized(int xOffset, int yOffset, int w, int h) {
	x = xOffset;
	y = yOffset;

	width = w;
	height = h;

}
