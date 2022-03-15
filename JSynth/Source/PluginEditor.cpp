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
    oscillatorsUI = new OscillatorsUI();

    setSize(640, 360);

    setResizable(true, true);
}

JSynthAudioProcessorEditor::~JSynthAudioProcessorEditor() {
}

//==============================================================================
void JSynthAudioProcessorEditor::paint(juce::Graphics& g) {
    // (Our component is opaque, so we must completely fill the background with a solid colour)
    g.fillAll(getLookAndFeel().findColour(juce::ResizableWindow::backgroundColourId));

    g.setColour(juce::Colour::fromRGB(255, 255, 255));

    g.setFont(32);

    g.drawFittedText("JSynth", 8, 8, 512, 32, juce::Justification::left, 1);

    oscillatorsUI->draw(g);
}

void JSynthAudioProcessorEditor::resized() {
    int width = getWidth();
    int height = getHeight();

    int lowerPartHeight = height - 48;

    oscillatorsUI->resized(0, 48, width / 2, lowerPartHeight / 2);
}
