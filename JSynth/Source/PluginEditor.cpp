/*
  ==============================================================================

    This file contains the basic framework code for a JUCE plugin editor.

  ==============================================================================
*/

#include "PluginProcessor.h"
#include "PluginEditor.h"

//==============================================================================
JSynthAudioProcessorEditor::JSynthAudioProcessorEditor(JSynthAudioProcessor& p)
    : AudioProcessorEditor(&p), audioProcessor(p)
{
    oscillatorsUI = new OscillatorsUI(p, this);
    envelopeUI = new EnvelopeUI(p, this);
    filtersUI = new FiltersUI(p, this);
    lfoUI = new LFOUI(p, this);

    setSize(640, 360);

    setResizeLimits(600, 340, 1920, 1080);

    setResizable(true, true);
}

JSynthAudioProcessorEditor::~JSynthAudioProcessorEditor() {
}

//==============================================================================
void JSynthAudioProcessorEditor::paint(juce::Graphics& g) {
    // (Our component is opaque, so we must completely fill the background with a solid colour)
    g.fillAll(juce::Colour::fromRGB(240, 240, 240));

    g.setColour(juce::Colour::fromRGB(32, 32, 32));

    g.setFont(32);

    g.drawFittedText("JSynth", 8, 8, 512, 32, juce::Justification::left, 1);

    oscillatorsUI->draw(g);
    envelopeUI->draw(g);
    filtersUI->draw(g);
    lfoUI->draw(g);
}

void JSynthAudioProcessorEditor::resized() {
    int width = getWidth();
    int height = getHeight();

    int lowerPartHeight = height - 48;

    oscillatorsUI->resized(0, 48, width, lowerPartHeight / 2);
    envelopeUI->resized(0, 48 + (lowerPartHeight / 2), width / 3, lowerPartHeight / 2);
    filtersUI->resized(width / 3, 48 + (lowerPartHeight / 2), width / 3, lowerPartHeight / 2);
    lfoUI->resized(2 * width / 3, 48 + (lowerPartHeight / 2), width / 3, lowerPartHeight / 2);
}
