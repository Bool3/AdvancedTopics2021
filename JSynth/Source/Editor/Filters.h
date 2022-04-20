
#pragma once

#include <JuceHeader.h>

class FiltersUI {
public:
	FiltersUI();
	~FiltersUI();

	void draw(juce::Graphics& g);
	void resized(int xOffset, int yOffset, int width, int height);

private:
	int x;
	int y;
	int width;
	int height;
};

