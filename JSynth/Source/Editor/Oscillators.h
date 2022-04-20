
#pragma once

#include <JuceHeader.h>

#include "OscillatorUI/OscillatorUI.h"

class OscillatorsUI {
public:
	OscillatorsUI();
	~OscillatorsUI();

	void draw(juce::Graphics& g);
	void resized(int xOffset, int yOffset, int width, int height);

private:
	int x;
	int y;
	int width;
	int height;

	OscillatorUI* oscillatorUI1;
	OscillatorUI* oscillatorUI2;
};
