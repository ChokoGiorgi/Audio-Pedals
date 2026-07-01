#include "PedalBoard.hpp"
#include "properties.hpp"
#include <span>
#include <algorithm>

/*
AURenderCallback inputProc - function pointer to audio processing callback
void* inputProcRefCon - raw pointer to any custom data passed into callback
*/
void PedalBoard::setupAudioUnit(AudioUnit audioUnit) {
    AURenderCallbackStruct callbackStruct;
    callbackStruct.inputProcRefCon = this;
    callbackStruct.inputProc = RenderCallback;

    /*
    AudioUnit inUnit - Audio Unit of modification
    AudioUnitPropertyID inID - what exactly should be changed
    AudioUnitScope inScope - where it should be changed (Input/Output/Global)
    AudioUnitElement inElement - which audio bus should be modified
    const void* inData - what data is fed to Audio Unit
    UInt32 inDataSize - size of fed data (inData)
    */
    AudioUnitSetProperty(
        audioUnit,
        kAudioUnitProperty_SetRenderCallback,
        kAudioUnitScope_Global,
        0,
        &callbackStruct,
        sizeof(callbackStruct)
    );
}

OSStatus PedalBoard::RenderCallback(
    void* inRefCon,
    AudioUnitRenderActionFlags* ioActionFlags,
    const AudioTimeStamp* inTimeStamp,
    [[maybe_unused]] UInt32 inBusNumber,
    UInt32 inNumberFrames,
    AudioBufferList* ioData)
{
    auto* engine = static_cast<PedalBoard*>(inRefCon);
    return engine->renderAudio(ioActionFlags, inTimeStamp, inNumberFrames, ioData);
}

OSStatus PedalBoard::renderAudio(
    [[maybe_unused]] AudioUnitRenderActionFlags* ioActionFlags,
    [[maybe_unused]] const AudioTimeStamp* inTimeStamp,
    [[maybe_unused]] UInt32 inNumberFrames,
    AudioBufferList* ioData)
{
    std::span<AudioBuffer> buffers(ioData->mBuffers, ioData->mNumberBuffers);

    for(AudioBuffer& buffer : buffers) {
        auto* floatData = static_cast<float*>(buffer.mData);
        size_t numFloats = buffer.mDataByteSize / sizeof(float);
        std::span<float> audioSamples(floatData, numFloats);
        std::fill(audioSamples.begin(), audioSamples.end(), 0.0f); //silence
    }

    return noErr;
}