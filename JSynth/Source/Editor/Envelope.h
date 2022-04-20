
#pragma once

#include <JuceHeader.h>

class EnvelopeUI {
public:
	EnvelopeUI();
	~EnvelopeUI();

	void draw(juce::Graphics& g);
	void resized(int xOffset, int yOffset, int width, int height);

private:
	int x;
	int y;
	int width;
	int height;
};

