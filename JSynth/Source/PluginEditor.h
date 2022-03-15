/*
  ==============================================================================

    This file contains the basic framework code for a JUCE plugin editor.

  ==============================================================================
*/

#pragma once

#include <JuceHeader.h>
#include "PluginProcessor.h"

#include "Editor/Oscillators.h"

//==============================================================================
/**
*/
class JSynthAudioProcessorEditor : public juce::AudioProcessorEditor {
public:
    JSynthAudioProcessorEditor(JSynthAudioProcessor&);
    ~JSynthAudioProcessorEditor() override;

    //==============================================================================
    void paint(juce::Graphics&) override;
    void resized() override;

private:
    // This reference is provided as a quick way for your editor to
    // access the processor object that created it.
    JSynthAudioProcessor& audioProcessor;

    OscillatorsUI* oscillatorsUI;

    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR(JSynthAudioProcessorEditor)
};
