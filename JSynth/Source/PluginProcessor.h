/*
  ==============================================================================

    This file contains the basic framework code for a JUCE plugin processor.

  ==============================================================================
*/

#pragma once

#include <JuceHeader.h>

extern class JProcessor;

//==============================================================================
/**
*/
class JSynthAudioProcessor : public juce::AudioProcessor {
public:
    //==============================================================================
    JSynthAudioProcessor();
    ~JSynthAudioProcessor() override;

    //==============================================================================
    void prepareToPlay(double sampleRate, int samplesPerBlock) override;
    void releaseResources() override;

   #ifndef JucePlugin_PreferredChannelConfigurations
    bool isBusesLayoutSupported(const BusesLayout& layouts) const override;
   #endif

    void processBlock(juce::AudioBuffer<float>&, juce::MidiBuffer&) override;

    //==============================================================================
    juce::AudioProcessorEditor* createEditor() override;
    bool hasEditor() const override;

    //==============================================================================
    const juce::String getName() const override;

    bool acceptsMidi() const override;
    bool producesMidi() const override;
    bool isMidiEffect() const override;
    double getTailLengthSeconds() const override;

    //==============================================================================
    int getNumPrograms() override;
    int getCurrentProgram() override;
    void setCurrentProgram(int index) override;
    const juce::String getProgramName(int index) override;
    void changeProgramName(int index, const juce::String& newName) override;

    //==============================================================================
    void getStateInformation(juce::MemoryBlock& destData) override;
    void setStateInformation(const void* data, int sizeInBytes) override;

    juce::AudioParameterChoice* wave1;
    juce::AudioParameterFloat* volume1;
    juce::AudioParameterInt* detuneSemitones1;
    juce::AudioParameterInt* detuneCents1;

    juce::AudioParameterChoice* wave2;
    juce::AudioParameterFloat* volume2;
    juce::AudioParameterInt* detuneSemitones2;
    juce::AudioParameterInt* detuneCents2;

    juce::AudioParameterFloat* attack;
    juce::AudioParameterFloat* decay;
    juce::AudioParameterFloat* sustain;
    juce::AudioParameterFloat* release;

    juce::AudioParameterInt* pitchBendLimit;

    juce::AudioParameterFloat* lfoFrequency;
    juce::AudioParameterChoice* lfoWave;
    juce::AudioParameterFloat* lfoIntensity;
    juce::AudioParameterChoice* lfoRoute;

    juce::AudioParameterChoice* filterType;
    juce::AudioParameterFloat* filterCutoffFrequency;
    juce::AudioParameterFloat* filterQFactor;

private:

    JProcessor* processors[2];

    //==============================================================================
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR(JSynthAudioProcessor)
};
